export const ROUNDS = 16;
export const BLOCK_SIZE = 8; // bytes (64-bit block)
export const KEY_SIZE = 8; // bytes (64-bit key)

export class FeistelCipher {
  private subkeys: Uint32Array;

  constructor(_key: Uint8Array) {
    this.subkeys = new Uint32Array(ROUNDS);
  }
}
