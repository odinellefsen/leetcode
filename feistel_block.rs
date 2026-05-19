pub const ROUNDS: usize = 16;
pub const BLOCK_SIZE: usize = 8; // bytes (64-bit block)

pub struct FeistelCipher {
    subkeys: [u32; ROUNDS],
}

impl FeistelCipher {
    pub fn new(key: [u8; 16]) -> Self {
        Self { subkeys: derive_subkeys(key) }
    }
}

// Derive ROUNDS 32-bit subkeys from a 128-bit master key.
//
// The 128-bit key is split into four 32-bit words k0..k3. Successive
// subkeys are produced by rotating the previous one and XORing in a
// round-dependent constant, giving every round a distinct key material.
fn derive_subkeys(key: [u8; 16]) -> [u32; ROUNDS] {
    let k: [u32; 4] = std::array::from_fn(|i| {
        u32::from_be_bytes(key[i * 4..i * 4 + 4].try_into().unwrap())
    });

    let mut subkeys = [0u32; ROUNDS];
    for i in 0..ROUNDS {
        let base = k[i % 4];
        let prev = if i == 0 { 0 } else { subkeys[i - 1] };
        subkeys[i] = base
            .rotate_left((i as u32 * 3 + 5) % 32)
            ^ prev
            ^ (i as u32).wrapping_mul(0x9e3779b9);
    }
    subkeys
}

// The round function F — the only cipher-specific component.
//
// Everything around F (split, XOR, swap, merge) is identical in every
// Feistel cipher; only F differentiates one design from another.
//
// This F is an ARX (Add-Rotate-XOR) mixing function:
//   1. Key whitening: XOR the half with the subkey.
//   2. Multiply by a near-golden-ratio constant to diffuse every bit.
//   3. Fold the high bits back in to break linearity.
//   4. A second multiply + fold for full avalanche.
fn round_fn(half: u32, subkey: u32) -> u32 {
    let x = half ^ subkey;
    let x = x.wrapping_mul(0x9e3779b9); // near-golden-ratio constant
    let x = x ^ (x >> 16);
    let x = x.wrapping_mul(0x85ebca6b);
    x ^ (x >> 13)
}

// Split an 8-byte block into two 32-bit halves (big-endian).
fn split_block(block: [u8; 8]) -> (u32, u32) {
    let l = u32::from_be_bytes(block[0..4].try_into().unwrap());
    let r = u32::from_be_bytes(block[4..8].try_into().unwrap());
    (l, r)
}

// Recombine two 32-bit halves back into an 8-byte block.
fn merge_halves(l: u32, r: u32) -> [u8; 8] {
    let mut block = [0u8; 8];
    block[0..4].copy_from_slice(&l.to_be_bytes());
    block[4..8].copy_from_slice(&r.to_be_bytes());
    block
}
