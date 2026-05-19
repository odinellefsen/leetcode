pub const ROUNDS: usize = 16;
pub const BLOCK_SIZE: usize = 8; // bytes (64-bit block)

pub struct FeistelCipher {
    subkeys: [u32; ROUNDS],
}

impl FeistelCipher {
    pub fn new(_key: [u8; 16]) -> Self {
        Self { subkeys: [0u32; ROUNDS] }
    }
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
