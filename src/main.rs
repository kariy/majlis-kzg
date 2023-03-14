use clap::{Parser, Subcommand};
use color_eyre::Result;
use reqwest::Url;
use sequencer::{types::TryContributeResponse, SequencerClient};

mod contribution;
mod prompt;
mod sequencer;

use prompt::{do_with_spinner, prompt_authentication};
use tokio::time;

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
        Commands::Start => {
            let session_id = prompt_authentication(&seq)?;

            // poll every 4 seconds
            loop {
                let res = seq.try_contribute(&session_id).await;
                match res {
                    Ok(contr) => match contr {
                        TryContributeResponse::InProgress(msg) => {
                            todo!("try contributino in progress")
                        }
                        TryContributeResponse::BatchContribution(_) => {
                            todo!("can contribute now");
                            return Ok(());
                        }
                    },
                    Err(e) => println!("error bro {e}"),
                }
                time::sleep(time::Duration::from_secs(4)).await;
            }
        }

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
