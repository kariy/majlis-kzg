use clap::{Parser, Subcommand};
use color_eyre::{eyre::bail, Result};
use reqwest::Url;
use sequencer::{types::TryContributeResponse, SequencerClient, SequencerErrorInner};
use tokio::time;

mod contribution;
mod prompt;
mod sequencer;

use prompt::{do_with_spinner, prompt_authentication, prompt_title};
use sequencer::{types::TryContributeError, SequencerClientError};

#[derive(Parser, Debug)]
struct App {
    #[arg(short, long)]
    #[arg(default_value = "https://seq.ceremony.ethereum.org")]
    sequencer_url: Url,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Start the ceremony.")]
    Start,
    #[command(about = "Get ceremony status.")]
    Status,
    #[command(about = "Request the current transcript.")]
    CurrentState,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::parse();
    let seq = SequencerClient::new(app.sequencer_url);

    match app.commands {
        Commands::Start => start_contribution(&seq).await?,

        Commands::Status => {
            let res = do_with_spinner(seq.status(), "Fetching status")?;
            println!(
                r#"
### Ceremony status ###
🎪 Lobby size: {}
📝 No. of contributions: {}
💻 Sequencer address: {}"#,
                res.lobby_size,
                res.num_contributions,
                res.sequencer_address.to_lowercase()
            )
        }

        Commands::CurrentState => {
            todo!("execute current state command")
        }
    }

    Ok(())
}

async fn start_contribution(sequencer: &SequencerClient) -> Result<()> {
    prompt_title();
    let session_id = prompt_authentication(sequencer)?;

    loop {
        let res = do_with_spinner(
            sequencer.try_contribute(&session_id),
            " Starting ceremony\n",
        );

        if let Err(err) = res {
            if let SequencerClientError::SequencerError(SequencerErrorInner { code, .. }) = err {
                let msg = match code {
                    TryContributeError::UnknownSessionId => {
                        "Invalid session ID. Please try authenticating again."
                    }
                    TryContributeError::RateLimited => "You are making too many requests.",
                };

                bail!(msg)
            } else {
                bail!(err);
            }
        }

        match res.unwrap() {
            TryContributeResponse::InProgress(msg) => println!("In progress... {msg}"),

            TryContributeResponse::BatchContribution(_) => {
                todo!("can contribute now");
            }
        }

        time::sleep(time::Duration::from_secs(4)).await;
    }
}
