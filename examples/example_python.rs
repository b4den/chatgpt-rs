use anyhow::Result;

use chatgpt_rs::client::GPTClient;

#[tokio::main]
async fn main() -> Result<()> {
    let message = "Write a bubble sort algorithm in Python";
    let mut gpt_client = GPTClient::new()?;
    let resp = gpt_client.post(message.to_string()).await?;
    println!("Done. \n{resp}");



    Ok(())
}
