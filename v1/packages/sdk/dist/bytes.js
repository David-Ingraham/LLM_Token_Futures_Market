/** Browser-safe byte helpers (no Node Buffer). */
const textEncoder = new TextEncoder();
export function utf8Bytes(s) {
    return textEncoder.encode(s);
}
export function u64le(value) {
    const buf = new Uint8Array(8);
    new DataView(buf.buffer).setBigUint64(0, value, true);
    return buf;
}
export function alloc(size) {
    return new Uint8Array(size);
}
export function writeU8(buf, offset, value) {
    buf[offset] = value;
    return offset + 1;
}
export function writeU64(buf, offset, value) {
    const view = new DataView(buf.buffer, buf.byteOffset, buf.byteLength);
    view.setBigUint64(offset, value, true);
    return offset + 8;
}
export function writeI64(buf, offset, value) {
    const view = new DataView(buf.buffer, buf.byteOffset, buf.byteLength);
    view.setBigInt64(offset, value, true);
    return offset + 8;
}
export function writeU16(buf, offset, value) {
    const view = new DataView(buf.buffer, buf.byteOffset, buf.byteLength);
    view.setUint16(offset, value, true);
    return offset + 2;
}
export function encodeInstruction(size, write) {
    const buf = alloc(size);
    const end = write(buf);
    return buf.slice(0, end);
}
export function readU64(data, offset) {
    return new DataView(data.buffer, data.byteOffset, data.byteLength).getBigUint64(offset, true);
}
export function readI64(data, offset) {
    return new DataView(data.buffer, data.byteOffset, data.byteLength).getBigInt64(offset, true);
}
export function readU16(data, offset) {
    return new DataView(data.buffer, data.byteOffset, data.byteLength).getUint16(offset, true);
}
export function readU8(data, offset) {
    return data[offset];
}
/** @solana/web3.js types expect Buffer; runtime accepts Uint8Array. */
export function ixData(bytes) {
    return bytes;
}
