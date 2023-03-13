use color_eyre::Result;
use reqwest::Url;
use sequencer::SequencerClient;

mod contribution;
mod sequencer;

#[tokio::main]
async fn main() -> Result<()> {
    let seq = SequencerClient::new(Url::parse("https://seq.ceremony.ethereum.org")?);
    let res = seq.status().await.unwrap();

    Ok(())
}
