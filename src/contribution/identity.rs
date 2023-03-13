use color_eyre::{eyre::ensure, Result};
use serde_json::Value;

pub fn eth_address_to_identity<T: AsRef<str>>(eth_address: T) -> Result<String> {
    let addr = eth_address.as_ref();
    let has_prefix = addr.strip_prefix("0x");

    ensure!(
        has_prefix.is_some(),
        "Ethereum Addresses must start with `0x`."
    );

    hex::decode(&has_prefix.unwrap())?;
    return Ok(format!("eth|{addr}"));
}

// this function assumes that the `@` in the handle is already removed
async fn get_github_id<T: AsRef<str>>(github_handle: T) -> Result<u64> {
    let handle = github_handle.as_ref();

    // GitHub API requires request to have valid `User-Agent` header `https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required`
    let client = reqwest::ClientBuilder::new().user_agent(handle).build()?;

    let res = client
        .get(format!("https://api.github.com/users/{handle}"))
        .send()
        .await?
        .error_for_status()?
        .json::<Value>()
        .await?;

    return res["id"]
        .as_u64()
        .ok_or(color_eyre::eyre::eyre!("field `id` doesn't exist"));
}

pub async fn github_handle_to_identity<T: AsRef<str>>(github_handle: T) -> Result<String> {
    let handle = github_handle.as_ref();
    let without_prefix = handle.strip_prefix('@');

    ensure!(
        without_prefix.is_some(),
        "GitHub handles must start with `@`."
    );

    let id = get_github_id(without_prefix.unwrap()).await?;
    return Ok(format!("git|{id}|{}", handle.to_lowercase()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_identity_from_eth_address() {
        let addr = "0x73F8A075b9a1e3ddD169CfdBdFA513c40B8bd796";
        let iden = eth_address_to_identity(addr).unwrap();
        assert_eq!(iden, format!("eth|{addr}"))
    }

    #[tokio::test]
    async fn create_identity_from_github_handle() {
        let handle = "@kariy";
        let iden = github_handle_to_identity(handle).await.unwrap();
        assert_eq!(iden, format!("git|26515232|{handle}"))
    }

    #[tokio::test]
    async fn create_identity_from_github_handle_without_prefix() {
        let handle = "kariy";
        assert!(github_handle_to_identity(handle).await.is_err())
    }
}
