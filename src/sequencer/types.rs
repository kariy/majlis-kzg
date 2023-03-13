use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::contribution::types::BatchContribution;

#[derive(Debug, Serialize, Deserialize)]
pub struct CeremonyStatus {
    pub lobby_size: i32,
    pub num_contributions: i32,
    pub sequencer_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub eth_auth_url: String,
    pub github_auth_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TryContributeResponse {
    BatchContribution(BatchContribution),
    InProgress(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionReceipt {
    receipt: String,
    signature: String,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum SessionError {
    #[error("SessionError::InvalidSessionId")]
    #[serde(rename = "SessionError::InvalidSessionId")]
    InvalidSessionId,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum TryContributeError {
    #[error("TryContributeError::RateLimited")]
    #[serde(rename = "TryContributeError::RateLimited")]
    RateLimited,
    #[error("TryContributeError::UnknownSessionId")]
    #[serde(rename = "TryContributeError::UnknownSessionId")]
    UnknownSessionId,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ContributeError {
    #[error("ContributeError::NotUsersTurn")]
    #[serde(rename = "ContributeError::NotUsersTurn")]
    NotUsersTurn,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum CeremoniesError {
    #[error("CeremoniesError::UnexpectedNumContributions")]
    #[serde(rename = "CeremoniesError::UnexpectedNumContributions")]
    UnexpectedNumContributions,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum CeremonyError {
    #[error("CeremonyError::UnsupportedNumG1Powers")]
    #[serde(rename = "CeremonyError::UnsupportedNumG1Powers")]
    UnsupportedNumG1Powers,
    #[error("CeremonyError::UnsupportedNumG2Powers")]
    #[serde(rename = "CeremonyError::UnsupportedNumG2Powers")]
    UnsupportedNumG2Powers,
    #[error("CeremonyError::UnexpectedNumG1Powers")]
    #[serde(rename = "CeremonyError::UnexpectedNumG1Powers")]
    UnexpectedNumG1Powers,
    #[error("CeremonyError::UnexpectedNumG2Powers")]
    #[serde(rename = "CeremonyError::UnexpectedNumG2Powers")]
    UnexpectedNumG2Powers,
    #[error("CeremonyError::InconsistentNumG2Powers")]
    #[serde(rename = "CeremonyError::InconsistentNumG2Powers")]
    InconsistentNumG2Powers,
    #[error("CeremonyError::UnsupportedMoreG2Powers")]
    #[serde(rename = "CeremonyError::UnsupportedMoreG2Powers")]
    UnsupportedMoreG2Powers,
    #[error("CeremonyError::InvalidG1Power")]
    #[serde(rename = "CeremonyError::InvalidG1Power")]
    InvalidG1Power,
    #[error("CeremonyError::InvalidG2Power")]
    #[serde(rename = "CeremonyError::InvalidG2Power")]
    InvalidG2Power,
    #[error("CeremonyError::ParserError")]
    #[serde(rename = "CeremonyError::ParserError")]
    ParserError,
    #[error("CeremonyError::InvalidPubKey")]
    #[serde(rename = "CeremonyError::InvalidPubKey")]
    InvalidPubKey,
    #[error("CeremonyError::InvalidWitnessProduct")]
    #[serde(rename = "CeremonyError::InvalidWitnessProduct")]
    InvalidWitnessProduct,
    #[error("CeremonyError::InvalidWitnessPubKey")]
    #[serde(rename = "CeremonyError::InvalidWitnessPubKey")]
    InvalidWitnessPubKey,
    #[error("CeremonyError::PubKeyPairingFailed")]
    #[serde(rename = "CeremonyError::PubKeyPairingFailed")]
    PubKeyPairingFailed,
    #[error("CeremonyError::G1PairingFailed")]
    #[serde(rename = "CeremonyError::G1PairingFailed")]
    G1PairingFailed,
    #[error("CeremonyError::G2PairingFailed")]
    #[serde(rename = "CeremonyError::G2PairingFailed")]
    G2PairingFailed,
    #[error("CeremonyError::ZeroPubkey")]
    #[serde(rename = "CeremonyError::ZeroPubkey")]
    ZeroPubkey,
    #[error("CeremonyError::ZeroG1")]
    #[serde(rename = "CeremonyError::ZeroG1")]
    ZeroG1,
    #[error("CeremonyError::ZeroG2")]
    #[serde(rename = "CeremonyError::ZeroG2")]
    ZeroG2,
    #[error("CeremonyError::InvalidG1FirstValue")]
    #[serde(rename = "CeremonyError::InvalidG1FirstValue")]
    InvalidG1FirstValue,
    #[error("CeremonyError::InvalidG2FirstValue")]
    #[serde(rename = "CeremonyError::InvalidG2FirstValue")]
    InvalidG2FirstValue,
    #[error("CeremonyError::InvalidG1One")]
    #[serde(rename = "CeremonyError::InvalidG1One")]
    InvalidG1One,
    #[error("CeremonyError::InvalidG2One")]
    #[serde(rename = "CeremonyError::InvalidG2One")]
    InvalidG2One,
    #[error("CeremonyError::InvalidG2Pubkey")]
    #[serde(rename = "CeremonyError::InvalidG2Pubkey")]
    InvalidG2Pubkey,
    #[error("CeremonyError::DuplicateG1")]
    #[serde(rename = "CeremonyError::DuplicateG1")]
    DuplicateG1,
    #[error("CeremonyError::DuplicateG2")]
    #[serde(rename = "CeremonyError::DuplicateG2")]
    DuplicateG2,
    #[error("CeremonyError::ContributionNoEntropy")]
    #[serde(rename = "CeremonyError::ContributionNoEntropy")]
    ContributionNoEntropy,
    #[error("CeremonyError::WitnessLengthMismatch")]
    #[serde(rename = "CeremonyError::WitnessLengthMismatch")]
    WitnessLengthMismatch,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
#[serde(untagged)]
pub enum ContributionError {
    #[error(transparent)]
    Session(SessionError),
    #[error(transparent)]
    Ceremony(CeremonyError),
    #[error(transparent)]
    Ceremonies(CeremoniesError),
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Serialize, Deserialize, Error)]
#[serde(untagged)]
pub enum ContributionAbortError {
    #[error(transparent)]
    Session(SessionError),
    #[error(transparent)]
    Contribute(ContributeError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let err1 = "\"CeremonyError::DuplicateG1\"";
        let err2 = "\"SessionError::InvalidSessionId\"";

        let variant1: ContributionError = serde_json::from_str(err1).unwrap();
        let variant2: ContributionAbortError = serde_json::from_str(err2).unwrap();

        assert_eq!(
            variant1,
            ContributionError::Ceremony(CeremonyError::DuplicateG1)
        );
        assert_eq!(
            variant2,
            ContributionAbortError::Session(SessionError::InvalidSessionId)
        );
    }
}
