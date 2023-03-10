use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PowersOfTau {
    #[serde(rename = "G1Powers")]
    pub g1_powers: Vec<String>,
    #[serde(rename = "G1Powers")]
    pub g2_powers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Witness {
    #[serde(rename = "potPubkeys")]
    pub pot_pubkeys: Vec<String>,
    #[serde(rename = "runningProducts")]
    pub running_products: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcript {
    #[serde(rename = "numG1Powers")]
    pub num_g1_powers: i32,
    #[serde(rename = "numG2Powers")]
    pub num_g2_powers: i32,
    #[serde(rename = "powersOfTau")]
    pub powers_of_tau: PowersOfTau,
    pub witness: Witness,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchTranscript {
    pub transcripts: Vec<Transcript>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contribution {
    pub num_g1_powers: i32,
    pub num_g2_powers: i32,
    pub pot_pubkey: String,
    pub powers_of_tau: PowersOfTau,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchContribution {
    pub contributions: Vec<Contribution>,
}

#[derive(Deserialize)]
pub enum TryContributeResponse {
    InProgress(String),
    BatchContribution(BatchContribution),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionReceipt {
    receipt: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionError {
    InvalidSessionId,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContributeError {
    NotUsersTurn,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CeremonyError {
    UnexpectedNumContributions,
    UnsupportedNumG1Powers,
    UnsupportedNumG2Powers,
    UnexpectedNumG1Powers,
    UnexpectedNumG2Powers,
    InconsistentNumG2Powers,
    UnsupportedMoreG2Powers,
    InvalidG1Power,
    InvalidG2Power,
    ParserError,
    InvalidPubKey,
    InvalidWitnessProduct,
    InvalidWitnessPubKey,
    PubKeyPairingFailed,
    G1PairingFailed,
    G2PairingFailed,
    ZeroPubkey,
    ZeroG1,
    ZeroG2,
    InvalidG1FirstValue,
    InvalidG2FirstValue,
    InvalidG1One,
    InvalidG2One,
    InvalidG2Pubkey,
    DuplicateG1,
    DuplicateG2,
    ContributionNoEntropy,
    WitnessLengthMismatch,
}

#[derive(Serialize, Deserialize)]
pub enum ContributionErrorCode {
    Session(SessionError),
    Ceremony(CeremonyError),
    Contribute(ContributeError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionError {
    code: CeremonyError,
    error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContributionAbortErrorCode {
    Session(SessionError),
    Contribute(ContributeError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionAbortError {
    code: ContributionAbortErrorCode,
    error: String,
}
