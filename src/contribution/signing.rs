#[cfg(feature = "eth")]
use super::types::BatchContribution;
#[cfg(feature = "eth")]
use ethers::core::types::transaction::eip712::TypedData;
#[cfg(feature = "eth")]
use serde_json::{json, Value};

use bls12_381::Scalar;
use blsful::{SecretKey, Signature};
use color_eyre::{eyre::ensure, Result};

use crate::contribution::identity::{eth_address_to_identity, github_handle_to_identity};

pub async fn sign_identity<T: AsRef<str>>(x: Scalar, identity: T) -> Result<Signature> {
    let identity = identity.as_ref();

    ensure!(
        identity.strip_prefix("0x").is_some() || identity.strip_prefix("@").is_some(),
        "unknown identity format"
    );

    let encoded_iden: Vec<u8> = if identity.strip_prefix("0x").is_some() {
        eth_address_to_identity(identity)?.as_bytes().into()
    } else {
        github_handle_to_identity(identity).await?.as_bytes().into()
    };

    let sk = SecretKey::from_bytes(&x.to_bytes()).unwrap();
    Ok(Signature::new(&sk, encoded_iden).unwrap())
}

#[cfg(feature = "eth")]
pub fn construct_contribution_eip712_typed_data(
    batch_contribution: &BatchContribution,
) -> Result<TypedData> {
    let mut keys: Vec<Value> = vec![];

    for contr in batch_contribution.contributions.iter() {
        keys.push(json!({
            "numG1Powers": contr.num_g1_powers,
            "numG2Powers": contr.num_g2_powers,
            "potPubkey": contr.pot_pubkey
        }));
    }

    let json = json!({
        "domain": {
            "name": "Ethereum KZG Ceremony",
            "version": "1.0",
            "chainId": 1
        },
        "types": {
            "EIP712Domain": [
                {"name":"name", "type":"string"},
                {"name":"version", "type":"string"},
                {"name":"chainId", "type":"uint256"}
            ],
            "contributionPubkey": [
                {"name": "numG1Powers", "type": "uint256"},
                {"name": "numG2Powers", "type": "uint256"},
                {"name": "potPubkey", "type": "bytes"}
            ],
            "PoTPubkeys": [
                { "name": "potPubkeys", "type": "contributionPubkey[]"}
            ]
        },
        "primaryType": "PoTPubkeys",
        "message": {
            "potPubkeys": keys
        }
    });

    serde_json::from_value(json).map_err(|e| color_eyre::eyre::eyre!(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    #[cfg(feature = "eth")]
    #[ignore]
    #[test]
    fn prepare_message_for_signing() {
        // let content = fs::read_to_string("initialContribution.json").unwrap();
        // let contribution = serde_json::from_str::<BatchContribution>(&content).unwrap();
        // let message = get_eip712_message_for_signing(contribution);

        // let expected = json!({
        //     "types": {
        //         "EIP712Domain": [
        //             {"name":"name", "type":"string"},
        //             {"name":"version", "type":"string"},
        //             {"name":"chainId", "type":"uint256"}
        //         ],
        //         "contributionPubkey": [
        //             {"name": "numG1Powers", "type": "uint256"},
        //             {"name": "numG2Powers", "type": "uint256"},
        //             {"name": "potPubkey", "type": "bytes"}
        //         ],
        //         "PoTPubkeys": [
        //             { "name": "potPubkeys", "type": "contributionPubkey[]"}
        //         ]
        //     },
        //     "primaryType": "PoTPubkeys",
        //     "domain": {
        //         "name": "Ethereum KZG Ceremony",
        //         "version": "1.0",
        //         "chainId": 1
        //     },
        //     "message": {
        //         "potPubkeys": [
        //             {
        //                 "numG1Powers": 4096,
        //                 "numG2Powers": 65,
        //                 "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
        //             },
        //             {
        //                 "numG1Powers": 8192,
        //                 "numG2Powers": 65,
        //                 "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
        //             },
        //             {
        //                 "numG1Powers": 16384,
        //                 "numG2Powers": 65,
        //                 "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
        //             },
        //             {
        //                 "numG1Powers": 32768,
        //                 "numG2Powers": 65,
        //                 "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
        //             }
        //         ]
        //     }
        // });

        // assert_eq!(message, expected.to_string())
    }
}
