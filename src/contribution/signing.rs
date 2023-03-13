use bls12_381::Scalar;
use serde_json::{json, Value};

use super::types::{BatchContribution, Contribution};

pub fn sign_identity(contribution: Contribution, x: Scalar, identity: &str) -> Contribution {
    unimplemented!("sign identity")
}

pub fn sign_contribution(
    batch_contribution: BatchContribution,
    ethereum_address: Option<String>,
) -> BatchContribution {
    unimplemented!("sign contribution")
}

pub fn get_eip712_message_for_signing(batch_contribution: BatchContribution) -> String {
    let mut keys: Vec<Value> = vec![];

    for contr in batch_contribution.contributions {
        keys.push(json!({
            "numG1Powers": contr.num_g1_powers,
            "numG2Powers": contr.num_g2_powers,
            "potPubkey": contr.pot_pubkey
        }));
    }

    let value = json!({
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
        "domain": {
            "name": "Ethereum KZG Ceremony",
            "version": "1.0",
            "chainId": 1
        },
        "message": {
            "potPubkeys": keys
        }
    });

    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    #[ignore]
    #[test]
    fn prepare_message_for_signing() {
        let content = fs::read_to_string("initialContribution.json").unwrap();
        let contribution = serde_json::from_str::<BatchContribution>(&content).unwrap();
        let message = get_eip712_message_for_signing(contribution);

        let expected = json!({
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
            "domain": {
                "name": "Ethereum KZG Ceremony",
                "version": "1.0",
                "chainId": 1
            },
            "message": {
                "potPubkeys": [
                    {
                        "numG1Powers": 4096,
                        "numG2Powers": 65,
                        "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
                    },
                    {
                        "numG1Powers": 8192,
                        "numG2Powers": 65,
                        "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
                    },
                    {
                        "numG1Powers": 16384,
                        "numG2Powers": 65,
                        "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
                    },
                    {
                        "numG1Powers": 32768,
                        "numG2Powers": 65,
                        "potPubkey": "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"
                    }
                ]
            }
        });

        assert_eq!(message, expected.to_string())
    }
}
