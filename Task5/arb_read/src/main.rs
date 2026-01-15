use anyhow::Result;
use ethers::prelude::*;

abigen!(Weth, "./src/abi.json");

#[tokio::main]
async fn main() -> Result<()> {
    // 0) 读取 .env（.env 里不要写 export）
    dotenvy::dotenv().ok();

    // 1) RPC
    let rpc = std::env::var("ARB_SEPOLIA_RPC")?;
    let provider = Provider::<Http>::try_from(rpc)?;

    // 1.5) 打印链 ID（Arbitrum Sepolia 应该是 421614）
    let chain_id = provider.get_chainid().await?;
    println!("chain_id = {}", chain_id);

    // 2) 合约地址：换成 Arbitrum Sepolia 上的 WETH（不要用以太坊 Sepolia 的 0xfff997...）
    // 方案 A：Arbiscan 上的 Wrapped Ether (WETH)
    let addr: Address = "0x802cc0f559ebc79da798bf3f3bab44141a1a06ed".parse()?;
    // 方案 B（可选）：Uniswap 文档给的 Arbitrum Sepolia Wrapped Native Token (WETH)
    // let addr: Address = "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73".parse()?;

    // 3) 强制校验：必须是合约
    let code = provider.get_code(addr, None).await?;
    if code.is_empty() {
        anyhow::bail!(
            "❌ 地址 {:?} 返回空代码。\n\
             1) 你连的链是否是 Arbitrum Sepolia (chainId=421614)?\n\
             2) 这个地址是否确实部署在 Arbitrum Sepolia 上？",
            addr
        );
    }

    // 4) 加载合约
    let contract = Weth::new(addr, provider.i
    println!("address  : {:?}", addr);
    println!("name     : {}", name);
    println!("symbol   : {}", symbol);
    println!("decimals : {}", decimals);

    Ok(())
}
nto());

    // 5) 只读调用
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;

    println!("✅ 合约存在，读取成功！");