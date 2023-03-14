use bls12_381::{G1Affine, G2Affine, Scalar};
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};
use rayon::prelude::*;

pub mod identity;
pub mod signing;
pub mod types;
pub mod utils;

use types::{BatchContribution, Contribution};
use utils::{bytes_from_hex_str, g1_point_from_compressed, g2_point_from_compressed};

pub fn generate_random_scalar() -> Scalar {
    let mut value = [0u8; 32];
    let mut rng = ChaCha8Rng::from_entropy();
    rng.fill_bytes(&mut value);
    Scalar::from_raw(unsafe { std::mem::transmute::<[u8; 32], [u64; 4]>(value) })
}

/// Subgroup checks :
/// - G1 Powers Subgroup check - For each of the Powers of Tau (g1_powers), verify that they are actually elements of the prime-ordered subgroup.
/// - G2 Powers Subgroup check - For each of the Powers of Tau (g2_powers), verify that they are actually elements of the prime-ordered subgroup.
/// - Running Product Subgroup check - Check that the last running product (the one the participant will interact with) is an element of the prime-ordered subgroup.
pub fn subgroup_checks(batch_contribution: BatchContribution) -> bool {
    let has_invalid = batch_contribution
        .contributions
        .iter()
        .par_bridge()
        .map(move |contr| {
            for g1_power in contr.powers_of_tau.g1_powers.iter() {
                let compressed = bytes_from_hex_str(g1_power);
                return !g1_point_from_compressed(&compressed).is_none();
            }

            for g2_power in contr.powers_of_tau.g2_powers.iter() {
                let compressed = bytes_from_hex_str(g2_power);
                return !g2_point_from_compressed(&compressed).is_none();
            }

            true
        })
        .any(|res| res == false);

    !has_invalid
}

// Perform the ceremony
pub fn update_batch(batch_contribution: &mut BatchContribution) {
    let mut contributions = batch_contribution.contributions.clone();

    contributions = contributions
        .into_iter()
        .par_bridge()
        .map(move |contr| {
            let x = generate_random_scalar();
            update_powers_of_tau(contr, x)
        })
        .collect::<Vec<Contribution>>();

    batch_contribution.contributions = contributions;
}

// TODO: for each contribution, the `x` scalar must be different
///  Updates the Powers of Tau within a sub-ceremony by multiplying each with a successive power of the secret x.
fn update_powers_of_tau(mut contribution: Contribution, x: Scalar) -> Contribution {
    let mut x_i: Scalar = Scalar::one();
    for i in 0..(contribution.num_g1_powers as usize) {
        let power1 = contribution.powers_of_tau.g1_powers[i].as_str();
        let point = g1_point_from_compressed(&bytes_from_hex_str(power1)).unwrap();

        let new_point = G1Affine::from(point * x_i);
        contribution.powers_of_tau.g1_powers[i] =
            format!("0x{}", hex::encode(new_point.to_compressed()));

        // update g2 powers
        if i < contribution.num_g2_powers as usize {
            let power2 = contribution.powers_of_tau.g2_powers[i].as_str();
            let point = g2_point_from_compressed(&bytes_from_hex_str(power2)).unwrap();

            let new_point = G2Affine::from(point * x_i);
            contribution.powers_of_tau.g2_powers[i] =
                format!("0x{}", hex::encode(new_point.to_compressed()));
        }

        x_i = x_i * x;
    }
    contribution
}

pub fn update_witness(contribution: &mut Contribution, x: Scalar) {
    let new_pot_pubkey = G2Affine::from(G2Affine::generator() * x);
    contribution.pot_pubkey = format!("0x{}", hex::encode(new_pot_pubkey.to_compressed()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[ignore]
    #[test]
    fn generate_g1point_from_random_key() {
        let power = "97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb";
        let point = g1_point_from_compressed(&bytes_from_hex_str(power)).unwrap();
        let compressed = hex::encode(point.to_compressed());
    }

    #[ignore]
    #[test]
    fn update_powers_of_tau() {
        let content = fs::read_to_string("initialTranscript.json").unwrap();
        let contribution = serde_json::from_str::<BatchContribution>(&content).unwrap();

        let random = generate_random_scalar();
        super::update_powers_of_tau(contribution.contributions[0].clone(), random);
    }

    #[ignore]
    #[test]
    fn check_subgroup_contribution_file() {
        let content = fs::read_to_string("initialTranscript.json").unwrap();
        let contribution = serde_json::from_str::<BatchContribution>(&content).unwrap();
        assert!(subgroup_checks(contribution));
    }
}
