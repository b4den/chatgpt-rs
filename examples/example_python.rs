use anyhow::Result;

use chatgpt_rs::client::GPTClient;

#[tokio::main]
async fn main() -> Result<()> {
    //let message = "Write bubble sort algorithm in Python";
    let message = "Which programming language should I use";
    //let message = "Who created Twitter";
    let mut gpt_client = GPTClient::new()?;
    let resp = gpt_client.post(message.to_string()).await?;
    println!("Done. Got {resp:?}");
    println!("Finished sending message: {message}");



    Ok(())
}
