// terminal: rustc --edition 2021 feistel_block.rs -o feistel_block && ./feistel_block

//! Simple Feistel block cipher.
//!
//! ## Design
//!
//! | Parameter  | Value                        |
//! |------------|------------------------------|
//! | Block size | 64 bits (two 32-bit halves)  |
//! | Key size   | 128 bits                     |
//! | Rounds     | 16                           |
//! | Mode       | ECB (demo only)              |
//!
//! A Feistel network is a symmetric structure that converts any function `F`
//! into an invertible permutation without requiring `F` itself to be
//! invertible. Each round:
//!
//! ```text
//!   L_i+1 = R_i
//!   R_i+1 = L_i  ⊕  F(R_i, K_i)
//! ```
//!
//! Decryption reuses the exact same network with subkeys in reverse order —
//! `F` is never inverted. This is the defining elegance of the construction.
//!
//! ## Round function (`F`)
//!
//! `F` is the **only** cipher-specific piece. Everything else — the split,
//! the XOR, the swap, the merge — is generic Feistel boilerplate. This
//! implementation uses an ARX (Add-Rotate-XOR / multiply-XOR) mixer drawn
//! from Murmur/Xorshift lineage for fast, well-tested avalanche behaviour.

pub const ROUNDS: usize = 16;
pub const BLOCK_SIZE: usize = 8; // bytes (64-bit block)

pub struct FeistelCipher {
    subkeys: [u32; ROUNDS],
}

impl FeistelCipher {
    pub fn new(key: [u8; 16]) -> Self {
        Self {
            subkeys: derive_subkeys(key),
        }
    }

    // Encrypt a single 64-bit block.
    //
    // Standard Feistel network with ROUNDS rounds:
    //   each round applies F to the right half and XORs the result into
    //   the left half, then swaps the halves.
    pub fn encrypt_block(&self, block: [u8; 8]) -> [u8; 8] {
        let (mut l, mut r) = split_block(block);
        for i in 0..ROUNDS {
            let new_r = l ^ round_fn(r, self.subkeys[i]);
            l = r;
            r = new_r;
        }
        merge_halves(l, r)
    }

    // Decrypt a single 64-bit block.
    //
    // Because F never needs to be invertible, decryption is identical to
    // encryption — just apply the subkeys in reverse order. This is the
    // elegant property that makes the Feistel construction so appealing.
    pub fn decrypt_block(&self, block: [u8; 8]) -> [u8; 8] {
        let (mut l, mut r) = split_block(block);
        for i in (0..ROUNDS).rev() {
            let new_l = r ^ round_fn(l, self.subkeys[i]);
            r = l;
            l = new_l;
        }
        merge_halves(l, r)
    }
}

// Derive ROUNDS 32-bit subkeys from a 128-bit master key.
//
// The 128-bit key is split into four 32-bit words k0..k3. Successive
// subkeys are produced by rotating the previous one and XORing in a
// round-dependent constant, giving every round a distinct key material.
fn derive_subkeys(key: [u8; 16]) -> [u32; ROUNDS] {
    let k: [u32; 4] =
        std::array::from_fn(|i| u32::from_be_bytes(key[i * 4..i * 4 + 4].try_into().unwrap()));

    let mut subkeys = [0u32; ROUNDS];
    for i in 0..ROUNDS {
        let base = k[i % 4];
        let prev = if i == 0 { 0 } else { subkeys[i - 1] };
        subkeys[i] =
            base.rotate_left((i as u32 * 3 + 5) % 32) ^ prev ^ (i as u32).wrapping_mul(0x9e3779b9);
    }
    subkeys
}

// ECB (Electronic Code Book) mode: each block encrypted independently.
// Suitable for demonstration; not semantically secure for real use.
pub fn encrypt_ecb(cipher: &FeistelCipher, data: &[u8]) -> Vec<u8> {
    pkcs7_pad(data)
        .chunks(BLOCK_SIZE)
        .flat_map(|chunk| cipher.encrypt_block(chunk.try_into().unwrap()))
        .collect()
}

pub fn decrypt_ecb(cipher: &FeistelCipher, data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() % BLOCK_SIZE != 0 {
        return Err("ciphertext length not a multiple of block size");
    }
    let raw: Vec<u8> = data
        .chunks(BLOCK_SIZE)
        .flat_map(|chunk| cipher.decrypt_block(chunk.try_into().unwrap()))
        .collect();
    pkcs7_unpad(&raw)
}

// PKCS#7 padding: append N bytes each with value N so the total length
// is a multiple of BLOCK_SIZE. When the input is already aligned a full
// block of padding is added so unpadding is always unambiguous.
fn pkcs7_pad(data: &[u8]) -> Vec<u8> {
    let pad_len = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    let mut padded = data.to_vec();
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    padded
}

fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    let &pad_byte = data.last().ok_or("empty ciphertext")?;
    let pad_len = pad_byte as usize;
    if pad_len == 0 || pad_len > BLOCK_SIZE {
        return Err("invalid padding");
    }
    let payload_len = data.len().checked_sub(pad_len).ok_or("invalid padding")?;
    if data[payload_len..].iter().any(|&b| b != pad_byte) {
        return Err("invalid padding");
    }
    Ok(data[..payload_len].to_vec())
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

fn main() {
    let key: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ];
    let cipher = FeistelCipher::new(key);

    // ── single block ──────────────────────────────────────────────────────────
    let plaintext_block: [u8; 8] = *b"Feistel!";
    let ciphertext_block = cipher.encrypt_block(plaintext_block);
    let recovered_block = cipher.decrypt_block(ciphertext_block);

    println!("=== Single block ===");
    println!(
        "Plaintext : {:?}  (\"{}\")",
        plaintext_block,
        std::str::from_utf8(&plaintext_block).unwrap()
    );
    println!("Ciphertext: {:?}", ciphertext_block);
    println!(
        "Recovered : {:?}  (\"{}\")",
        recovered_block,
        std::str::from_utf8(&recovered_block).unwrap()
    );

    // ── multi-block ECB ───────────────────────────────────────────────────────
    let message = b"Hello from the Feistel cipher!";
    let encrypted = encrypt_ecb(&cipher, message);
    let decrypted = decrypt_ecb(&cipher, &encrypted).unwrap();

    println!("\n=== ECB mode ===");
    println!("Message  : \"{}\"", std::str::from_utf8(message).unwrap());
    println!(
        "Encrypted: {}",
        encrypted
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
    println!(
        "Decrypted: \"{}\"",
        std::str::from_utf8(&decrypted).unwrap()
    );

    // ── bit-flip sensitivity (avalanche) ─────────────────────────────────────
    let block_a: [u8; 8] = [0u8; 8];
    let mut block_b = block_a;
    block_b[0] ^= 0x01; // flip one bit

    let ct_a = cipher.encrypt_block(block_a);
    let ct_b = cipher.encrypt_block(block_b);
    let differing_bits: u32 = ct_a
        .iter()
        .zip(ct_b.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum();

    println!("\n=== Avalanche (1-bit input flip) ===");
    println!("CT_a: {:?}", ct_a);
    println!("CT_b: {:?}", ct_b);
    println!("Bits flipped in ciphertext: {}/64", differing_bits);
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ];

    #[test]
    fn block_round_trip() {
        let cipher = FeistelCipher::new(KEY);
        let plaintext = [0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe];
        assert_eq!(
            cipher.decrypt_block(cipher.encrypt_block(plaintext)),
            plaintext
        );
    }

    #[test]
    fn all_zero_block() {
        let cipher = FeistelCipher::new(KEY);
        let block = [0u8; 8];
        assert_eq!(cipher.decrypt_block(cipher.encrypt_block(block)), block);
    }

    #[test]
    fn all_ones_block() {
        let cipher = FeistelCipher::new(KEY);
        let block = [0xffu8; 8];
        assert_eq!(cipher.decrypt_block(cipher.encrypt_block(block)), block);
    }

    #[test]
    fn ecb_round_trip_aligned() {
        let cipher = FeistelCipher::new(KEY);
        let msg = b"ABCDEFGH"; // exactly one block
        let ct = encrypt_ecb(&cipher, msg);
        assert_eq!(decrypt_ecb(&cipher, &ct).unwrap(), msg);
    }

    #[test]
    fn ecb_round_trip_unaligned() {
        let cipher = FeistelCipher::new(KEY);
        let msg = b"Hello, Feistel!";
        let ct = encrypt_ecb(&cipher, msg);
        assert_eq!(decrypt_ecb(&cipher, &ct).unwrap(), msg);
    }

    #[test]
    fn different_keys_differ() {
        let key2 = [0xffu8; 16];
        let c1 = FeistelCipher::new(KEY);
        let c2 = FeistelCipher::new(key2);
        let block = [0u8; 8];
        assert_ne!(c1.encrypt_block(block), c2.encrypt_block(block));
    }

    #[test]
    fn ciphertext_differs_from_plaintext() {
        let cipher = FeistelCipher::new(KEY);
        let block = [0x11u8; 8];
        assert_ne!(cipher.encrypt_block(block), block);
    }
}
