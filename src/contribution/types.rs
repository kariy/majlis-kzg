use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptPowersOfTau {
    #[serde(rename = "G1Powers")]
    pub g1_powers: Vec<String>,
    #[serde(rename = "G2Powers")]
    pub g2_powers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Witness {
    #[serde(rename = "runningProducts")]
    pub running_products: Vec<String>,
    #[serde(rename = "potPubkeys")]
    pub pot_pubkeys: Vec<String>,
    #[serde(rename = "blsSignatures")]
    pub bls_signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcript {
    #[serde(rename = "numG1Powers")]
    pub num_g1_powers: i32,
    #[serde(rename = "numG2Powers")]
    pub num_g2_powers: i32,
    #[serde(rename = "powersOfTau")]
    pub powers_of_tau: TranscriptPowersOfTau,
    pub witness: Witness,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchTranscript {
    pub transcripts: Vec<Transcript>,
    #[serde(rename = "participantIds")]
    pub participant_ids: Vec<String>,
    #[serde(rename = "participantEcdsaSignatures")]
    pub participant_ecds_signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PowersOfTau {
    pub g1_powers: Vec<String>,
    pub g2_powers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contribution {
    pub num_g1_powers: i32,
    pub num_g2_powers: i32,
    pub powers_of_tau: PowersOfTau,
    pub pot_pubkey: String,
    pub bls_signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchContribution {
    pub contributions: Vec<Contribution>,
    pub ecdsa_signature: Option<String>,
}
