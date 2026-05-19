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
