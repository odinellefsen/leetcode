/**
 * Partial AES-128 implementation.
 *
 * This file intentionally implements only a subset of AES:
 * block/key sizing, byte validation, and small utility helpers. Later commits
 * add the core round primitives and enough orchestration to run partial rounds.
 */

export const BLOCK_SIZE = 16;
export const KEY_SIZE_128 = 16;
export const AES_128_ROUNDS = 10;

export type AesBlock = Uint8Array;
export type AesKey128 = Uint8Array;

export function assertBlock(block: Uint8Array): void {
  assertLength("block", block, BLOCK_SIZE);
}

export function assertKey128(key: Uint8Array): void {
  assertLength("key", key, KEY_SIZE_128);
}

export function cloneBytes(bytes: Uint8Array): Uint8Array {
  return new Uint8Array(bytes);
}

export function xtime(byte: number): number {
  const shifted = (byte << 1) & 0xff;
  return (byte & 0x80) === 0 ? shifted : shifted ^ 0x1b;
}

export function gfMul(left: number, right: number): number {
  let a = left & 0xff;
  let b = right & 0xff;
  let product = 0;

  for (let i = 0; i < 8; i++) {
    if ((b & 1) !== 0) {
      product ^= a;
    }
    a = xtime(a);
    b >>>= 1;
  }

  return product & 0xff;
}

function assertLength(label: string, bytes: Uint8Array, expected: number): void {
  if (bytes.length !== expected) {
    throw new Error(`${label} must be ${expected} bytes`);
  }
}
