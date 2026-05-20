export const ROUNDS = 16;
export const BLOCK_SIZE = 8; // bytes (64-bit block)
export const KEY_SIZE = 8; // bytes (64-bit key)

export class FeistelCipher {
  private subkeys: Uint32Array;

  constructor(key: Uint8Array) {
    if (key.length !== KEY_SIZE) {
      throw new Error(`key must be ${KEY_SIZE} bytes`);
    }
    this.subkeys = deriveSubkeys(key);
  }
}

// Derive ROUNDS 32-bit subkeys from a 64-bit master key.
//
// The 64-bit key is split into two 32-bit words k0 and k1. A rotating
// state register is mixed with both words and a round constant to
// produce a distinct subkey for every round.
function deriveSubkeys(key: Uint8Array): Uint32Array {
  const k0 = readU32BE(key, 0);
  const k1 = readU32BE(key, 4);
  const subkeys = new Uint32Array(ROUNDS);
  let state = (k0 ^ k1) >>> 0;

  for (let i = 0; i < ROUNDS; i++) {
    state = ((state << 7) | (state >>> 25)) >>> 0;
    subkeys[i] = (k0 ^ k1 ^ state ^ (i * 0x6c078965)) >>> 0;
  }
  return subkeys;
}

function readU32BE(bytes: Uint8Array, offset: number): number {
  return (
    ((bytes[offset] << 24) |
      (bytes[offset + 1] << 16) |
      (bytes[offset + 2] << 8) |
      bytes[offset + 3]) >>>
    0
  );
}

function writeU32BE(value: number, bytes: Uint8Array, offset: number): void {
  bytes[offset] = (value >>> 24) & 0xff;
  bytes[offset + 1] = (value >>> 16) & 0xff;
  bytes[offset + 2] = (value >>> 8) & 0xff;
  bytes[offset + 3] = value & 0xff;
}

// Split an 8-byte block into two 32-bit halves (big-endian).
function splitBlock(block: Uint8Array): [number, number] {
  return [readU32BE(block, 0), readU32BE(block, 4)];
}

// Recombine two 32-bit halves back into an 8-byte block.
function mergeHalves(l: number, r: number): Uint8Array {
  const block = new Uint8Array(BLOCK_SIZE);
  writeU32BE(l, block, 0);
  writeU32BE(r, block, 4);
  return block;
}
