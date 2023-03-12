use dotenv::dotenv;
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let rpc_url = std::env::var("RPC_WS").expect("RPC_WS must be set.");

    let provider = Provider::<Ws>::connect(rpc_url).await?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("{block_number}");

    // Attempt to get an IP address and print it.
    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
    } else {
        println!("couldn't get an IP address");
    }

    Ok(())
}
