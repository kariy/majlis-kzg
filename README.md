# Majlis-KZG

This is a client implementation for participating in the Ethereum KZG Ceremony. It is written in Rust 🦀 so you know it is *BLAZINGLY* fast.

## Overview

The KZG Ceremony is a coordinated public ritual which will provide a cryptographic foundation for Ethereum scaling efforts like EIP-4844 (aka proto-danksharding). These types of events are also known as "Trusted Setups," famously used by Zcash to bootstrap the chain's privacy features. However, they can also be used to support scaling mechanisms, as Ethereum plans to do.

Proto-danksharding requires a new cryptographic scheme: KZG Commitments. These will generate a "structured reference string" (SRS) which is needed for the commitments to work. An SRS is secure as long as a single ceremony participant successfully conceals their secret.

It's a multi-party ceremony: each contributor creates a secret and runs a computation to mix it with previous contributions. Then, the output is made public and passed to the next contributor. The final output will be included in a future upgrade to help scale the Ethereum network.

## Cryptographic libraries

- [`ChaCha20`](https://rust-random.github.io/rand/rand_chacha/struct.ChaCha20Rng.html) CSPRNG for generating the secrets
- [`bls12_381`](https://docs.rs/bls12_381/latest/bls12_381/) BLS12-381 curve
- [`blsful`](https://docs.rs/blsful/1.1.1/blsful/) BLS Signature over the BLS12-381 curve


