use anyhow::Result;

use chatgpt_rs::client::GPTClient;

#[tokio::main]
async fn main() -> Result<()> {
    let message = "Draft a polite formal email to Alice apologising that Bob is unable to make Tuesdays meeting at 1:30PM";
    let mut gpt_client = GPTClient::new()?;
    let resp = gpt_client.post(message.to_string()).await?;
    println!("Done. \n{resp}");

    Ok(())
}
