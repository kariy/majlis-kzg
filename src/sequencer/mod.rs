use color_eyre::Result;
use reqwest::{get, Client, RequestBuilder};
use url::Url;

pub mod types;

use types::{AuthResponse, BatchTranscript, CeremonyStatus, TryContributeResponse};

use self::types::{BatchContribution, ContributionError, ContributionReceipt};

pub struct SequencerClient {
    url: Url,
}

impl SequencerClient {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub async fn status(&self) -> Result<CeremonyStatus> {
        let res = get(format!("{}info/status", self.url))
            .await?
            .error_for_status()?
            .json::<CeremonyStatus>()
            .await?;
        Ok(res)
    }

    pub async fn current_state(&self) -> Result<BatchTranscript> {
        let res = get(format!("{}info/current_state", self.url))
            .await?
            .error_for_status()?
            .json::<BatchTranscript>()
            .await?;
        Ok(res)
    }

    pub async fn request_auth_link(&self) -> Result<AuthResponse> {
        let res = get(format!("{}auth/request_link", self.url))
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn try_contribute(
        &self,
        session_id: impl AsRef<str>,
    ) -> Result<TryContributeResponse> {
        let res = self
            ._authenticated_post("/lobby/try_contribute", session_id)
            .send()
            .await?;

        Ok(res.json::<TryContributeResponse>().await?)
    }

    pub async fn contribute(
        &self,
        contributions: &BatchContribution,
        session_id: impl AsRef<str>,
    ) -> std::result::Result<ContributionReceipt, ContributionError> {
        let path = "contribute";

        let res = self
            ._authenticated_post(path, session_id)
            .json(contributions)
            .send()
            .await
            .unwrap();

        if res.status().is_success() {
            let data = res.json().await.unwrap();
            Ok(data)
        } else {
            let err = res.json().await.unwrap();
            Err(err)
        }
    }

    pub async fn abort_contribution(&self) -> Result<()> {
        let path = "/contribution/abort";
        todo!("abort_contribution needs implementation")
    }

    fn _authenticated_post<T, U>(&self, path: T, session_id: U) -> RequestBuilder
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        Client::new()
            .post(format!("{}{}", self.url, path.as_ref()))
            .bearer_auth(session_id.as_ref())
    }
}
