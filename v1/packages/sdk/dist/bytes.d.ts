/** Browser-safe byte helpers (no Node Buffer). */
export declare function utf8Bytes(s: string): Uint8Array;
export declare function u64le(value: bigint): Uint8Array;
export declare function alloc(size: number): Uint8Array;
export declare function writeU8(buf: Uint8Array, offset: number, value: number): number;
export declare function writeU64(buf: Uint8Array, offset: number, value: bigint): number;
export declare function writeI64(buf: Uint8Array, offset: number, value: bigint): number;
export declare function writeU16(buf: Uint8Array, offset: number, value: number): number;
export declare function encodeInstruction(size: number, write: (buf: Uint8Array) => number): Uint8Array;
export declare function readU64(data: Uint8Array, offset: number): bigint;
export declare function readI64(data: Uint8Array, offset: number): bigint;
export declare function readU16(data: Uint8Array, offset: number): number;
export declare function readU8(data: Uint8Array, offset: number): number;
/** @solana/web3.js types expect Buffer; runtime accepts Uint8Array. */
export declare function ixData(bytes: Uint8Array): Buffer;
