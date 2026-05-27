import { MICRO_PER_UNIT } from "./constants.js";

/** e.g. 25.0 -> 25_000_000 micro-dollars per MTok */
export function dollarsToMicro(dollars: number): number {
  if (!Number.isFinite(dollars) || dollars < 0) {
    throw new Error(`invalid dollars: ${dollars}`);
  }
  const micro = Math.round(dollars * MICRO_PER_UNIT);
  if (!Number.isSafeInteger(micro)) {
    throw new Error(`dollars out of safe integer range: ${dollars}`);
  }
  return micro;
}

export function microToDollars(micro: number): number {
  return micro / MICRO_PER_UNIT;
}

export function formatMicroUsd(micro: number): string {
  return `$${microToDollars(micro).toFixed(6)}`;
}
