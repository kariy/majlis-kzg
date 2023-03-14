use reqwest::{get, Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

pub mod types;

use crate::contribution::types::{BatchContribution, BatchTranscript};
use types::{AuthResponse, CeremonyStatus, ContributionReceipt, TryContributeResponse};
use types::{ContributionAbortError, ContributionError, TryContributeError};

#[derive(Debug, thiserror::Error)]
pub enum SequencerClientError<E> {
    #[allow(unused)]
    #[error(transparent)]
    JsonError(serde_json::Error),
    #[error(transparent)]
    TransportError(reqwest::Error),
    #[error(transparent)]
    SequencerError(SequencerErrorInner<E>),
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
#[error("Contribution error: code={code}, error=\"{error}\"")]
pub struct SequencerErrorInner<E> {
    code: E,
    error: String,
}

type Result<T, E> = std::result::Result<T, SequencerClientError<E>>;

pub struct SequencerClient {
    url: Url,
}

impl SequencerClient {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub async fn status(&self) -> Result<CeremonyStatus, String> {
        let res = get(format!("{}info/status", self.url)).await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            _ => unreachable!("unknown response code"),
        }
    }

    pub async fn current_state(&self) -> Result<BatchTranscript, String> {
        let res = get(format!("{}info/current_state", self.url)).await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            _ => unreachable!("unknown response code"),
        }
    }

    pub async fn request_auth_link(&self) -> Result<AuthResponse, String> {
        let res = get(format!("{}auth/request_link", self.url)).await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            _ => unreachable!("unknown response code"),
        }
    }

    pub async fn try_contribute<T: AsRef<str>>(
        &self,
        session_id: T,
    ) -> Result<TryContributeResponse, TryContributeError> {
        let res = self
            ._authenticated_post("lobby/try_contribute", session_id)
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED => {
                let err = res.json().await?;
                Err(SequencerClientError::SequencerError(err))
            }
            _ => unreachable!("unknown response code"),
        }
    }

    pub async fn contribute<T: AsRef<str>>(
        &self,
        contributions: &BatchContribution,
        session_id: T,
    ) -> Result<ContributionReceipt, ContributionError> {
        let res = self
            ._authenticated_post("contribute", session_id)
            .json(contributions)
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            StatusCode::BAD_REQUEST => {
                let err = res.json().await?;
                Err(SequencerClientError::SequencerError(err))
            }
            _ => unreachable!("unknown response code"),
        }
    }

    pub async fn abort_contribution<T: AsRef<str>>(
        &self,
        session_id: T,
    ) -> Result<(), ContributionAbortError> {
        let res = self
            ._authenticated_post("contribution/abort", session_id)
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            StatusCode::BAD_REQUEST => {
                let err = res.json().await?;
                Err(SequencerClientError::SequencerError(err))
            }
            _ => unreachable!("unknown response code"),
        }
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

impl<E> From<reqwest::Error> for SequencerClientError<E> {
    fn from(value: reqwest::Error) -> Self {
        Self::TransportError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{types::TryContributeError, *};
    use serde_json::json;

    #[test]
    fn test_deserialize_error() {
        let json = json!({
            "code": "TryContributeError::RateLimited",
            "error": "call came too early. rate limited"
        })
        .to_string();
        let expected = SequencerErrorInner {
            code: TryContributeError::RateLimited,
            error: "call came too early. rate limited".to_string(),
        };

        let err: SequencerErrorInner<TryContributeError> = serde_json::from_str(&json).unwrap();

        assert_eq!(err, expected);
    }
}
