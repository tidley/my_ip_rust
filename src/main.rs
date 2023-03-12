use dotenv::dotenv;
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let rpc_url = std::env::var("RPC_WS").expect("RPC_WS must be set.");

    let provider =  Provider::<Ws>::connect(rpc_url).await?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("{block_number}");

    Ok(())
}
