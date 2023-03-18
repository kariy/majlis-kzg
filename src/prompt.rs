use color_eyre::Result;
use indicatif::{ProgressBar, ProgressFinish};
use inquire::{Select, Text};
use std::{borrow::Cow, future::Future, time::Duration};

use crate::sequencer::SequencerClient;

#[cfg(not(feature = "eth"))]
const NO_AUTH_PROVIDERS: usize = 1;
#[cfg(feature = "eth")]
const NO_AUTH_PROVIDERS: usize = 2;
const AUTH_PROVIDERS: [&str; NO_AUTH_PROVIDERS] = [
    "GitHub",
    #[cfg(feature = "eth")]
    "Ethereum Address",
];

pub fn do_with_spinner<F, T>(future: F, message: impl Into<Cow<'static, str>>) -> T
where
    F: Future<Output = T>,
{
    let spinner = ProgressBar::new_spinner().with_message(message);
    spinner.enable_steady_tick(Duration::from_millis(100));
    let res = async_std::task::block_on(future);
    spinner.finish_and_clear();
    res
}

pub fn prompt_authentication(sequencer: &SequencerClient) -> Result<String> {
    let res = Select::new(
        "Select how you want to authenticate yourself.",
        AUTH_PROVIDERS.into(),
    )
    .prompt()
    .unwrap();

    let auth_links = do_with_spinner(sequencer.request_auth_link(), "")?;
    let link = match res {
        "GitHub" => auth_links.github_auth_url,
        #[cfg(feature = "eth")]
        "Ethereum Address" => auth_links.eth_auth_url,
        _ => unreachable!("unsupported auth provider"),
    };

    println!(
        r#"
Click the link below to authenticate and obtain your session ID :

{link}
"#,
    );

    let session_id = Text::new("Enter your session ID : ").prompt()?;
    println!("");
    Ok(session_id)
}

pub fn prompt_title() {
    let title = r#"
     __    __     ______       __     __         __     ______    
    /\ "-./  \   /\  __ \     /\ \   /\ \       /\ \   /\  ___\   
    \ \ \-./\ \  \ \  __ \   _\_\ \  \ \ \____  \ \ \  \ \___  \  
     \ \_\ \ \_\  \ \_\ \_\ /\_____\  \ \_____\  \ \_\  \/\_____\ 
      \/_/  \/_/   \/_/\/_/ \/_____/   \/_____/   \/_/   \/_____/ 
     __  __     ______     ______                                 
    /\ \/ /    /\___  \   /\  ___\                                
    \ \  _"-.  \/_/  /__  \ \ \__ \                               
     \ \_\ \_\   /\_____\  \ \_____\                              
      \/_/\/_/   \/_____/   \/_____/                              

"#;

    println!("{title}");
}
