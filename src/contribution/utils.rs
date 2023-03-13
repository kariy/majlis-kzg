use bls12_381::{G1Affine, G2Affine};

#[inline]
pub fn g1_point_from_compressed(compressed_g1_point: &[u8]) -> Option<G1Affine> {
    G1Affine::from_compressed(unsafe { &*(compressed_g1_point.as_ptr() as *const [u8; 48]) }).into()
}

#[inline]
pub fn g2_point_from_compressed(compressed_g2_point: &[u8]) -> Option<G2Affine> {
    G2Affine::from_compressed(unsafe { &*(compressed_g2_point.as_ptr() as *const [u8; 96]) }).into()
}

#[inline]
pub fn bytes_from_hex_str(hex: &str) -> Vec<u8> {
    hex::decode(hex.strip_prefix("0x").unwrap_or(hex)).expect("must able to parse hex string")
}
