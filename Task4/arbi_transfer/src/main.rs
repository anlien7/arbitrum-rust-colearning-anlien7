use ethers::prelude::*;
use std::{env, sync::Arc};
use anyhow::{Context, Result};
use dotenv::dotenv;
use ethers::types::transaction::eip2718::TypedTransaction;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // 1. 读配置
    let rpc = env::var("ARB_RPC")
        .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());
    let privkey_hex = env::var("PRIVKEY").context("请在 .env 设置 PRIVKEY")?;
    let to: Address = env::var("TO_ADDR")
        .context("请在 .env 设置 TO_ADDR")?
        .parse()
        .context("TO_ADDR 格式非法")?;
    let amount_eth: f64 = env::var("AMOUNT")
        .unwrap_or_else(|_| "0.001".to_string())
        .parse()
        .context("AMOUNT 必须是数字（单位 ETH）")?;
    let manual_gas_gwei = env::var("GAS_PRICE_GWEI")
        .ok()
        .and_then(|s| s.parse::<u64>().ok());

    // 2. 构建钱包 & 客户端
    let wallet: LocalWallet = privkey_hex
        .trim_start_matches("0x")
        .parse::<LocalWallet>()?
        .with_chain_id(421614u64);
    let provider = Provider::<Http>::try_from(rpc)?
        .interval(std::time::Duration::from_secs(1));
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let from = client.address();
    println!("发送地址: {:?}", from);
    println!("接收地址: {:?}", to);

    // 3. 余额检查
    let balance = client.get_balance(from, None).await?;
    let need_wei = ethers::utils::parse_ether(amount_eth)?;
    if balance < need_wei {
        anyhow::bail!(
            "余额不足：需要 {} ETH，实际 {} ETH",
            amount_eth,
            ethers::utils::format_ether(balance)
        );
    }

    // 4. GasPrice 策略
    let gas_price = if let Some(gwei) = manual_gas_gwei {
        ethers::utils::parse_units(gwei, "gwei")?.into()
    } else {
        let price = client.get_gas_price().await?;
        price * 110_u32 / 100_u32
    };
    println!("使用 gas_price: {} gwei", ethers::utils::format_units(gas_price, "gwei")?);

    // 5. 估算 gas
// 5. 估算 gas
    let tx_for_estimate = TransactionRequest::new()
        .to(to)
        .value(need_wei)
        .from(from);
    let typed = TypedTransaction::Legacy(tx_for_estimate);
    let estimated = client.estimate_gas(&typed, None).await?;
    let gas = estimated * 130_u32 / 100_u32;
    println!("估算 gas: {} → 使用 gas: {}", estimated, gas);

    // 6. 组装最终交易并发送
    let tx = TransactionRequest::new()
        .to(to)
        .value(need_wei)
        .gas_price(gas_price)
        .gas(gas)
        .from(from);

    let pending = client.send_transaction(tx, None).await?;
    let tx_hash = *pending;
    println!("已广播 → tx_hash: {:?}", tx_hash);

    let receipt = pending
        .confirmations(1)
        .await?
        .ok_or_else(|| anyhow::anyhow!("交易失败或被打回"))?;
    println!(
        "✅ 交易上链！block_number: {:?} | block_hash: {:?}",
        receipt.block_number,
        receipt.block_hash
    );

    Ok(())
}