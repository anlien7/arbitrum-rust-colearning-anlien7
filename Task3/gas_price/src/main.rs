use anyhow::{Context, Result};
use ethers::prelude::*;
use ethers::utils::format_ether;

const BASIC_TRANSFER_GAS_LIMIT: u64 = 21_000;

#[derive(Debug)]
pub struct GasEstimate {
    pub gas_price_wei: U256, // wei / gas
    pub gas_limit: U256,     // gas
    pub fee_wei: U256,       // wei
    pub fee_eth: String,     // formatted ETH
}

/// Gas 费计算逻辑：
/// 1) 通过 provider.get_gas_price() 从 Arbitrum 测试网 RPC 动态获取实时 Gas Price（单位：wei/gas），不做硬编码；
/// 2) 基础转账（EOA -> EOA）Gas Limit 采用行业通用值 21,000；
/// 3) 按公式计算预估费用：Gas Fee(wei) = Gas Price(wei/gas) × Gas Limit(gas)；
/// 4) fee_wei 为最终预估费用（wei），并用 format_ether 将 wei 转为 ETH 字符串便于展示。
pub async fn estimate_arb_transfer_gas_fee(rpc_url: &str) -> Result<GasEstimate> {
    let provider = Provider::<Http>::try_from(rpc_url)
        .with_context(|| format!("RPC URL 无法解析或初始化 Provider: {rpc_url}"))?;


    let gas_price_wei = provider
        .get_gas_price()
        .await
        .context("RPC 调用失败：get_gas_price")?;


    let gas_limit = U256::from(BASIC_TRANSFER_GAS_LIMIT);


    let fee_wei = gas_price_wei * gas_limit;


    let fee_eth = format_ether(fee_wei);

    Ok(GasEstimate {
        gas_price_wei,
        gas_limit,
        fee_wei,
        fee_eth,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";

    let est = estimate_arb_transfer_gas_fee(rpc_url).await?;

    println!("Gas price: {} wei/gas", est.gas_price_wei);
    println!("Gas limit: {} gas", est.gas_limit);
    println!("Estimated fee: {} wei (~{} ETH)", est.fee_wei, est.fee_eth);

    Ok(())
}
