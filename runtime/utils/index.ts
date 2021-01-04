import * as util from "util";
import * as crypto from "crypto";

import { wasm_exports } from "../index";

export const setValue = (value: string) => {
  const textEncoder = new util.TextEncoder();
  const typedArray = textEncoder.encode(value);
  const ptr = wasm_exports._wasm_malloc(typedArray.length);
  const Uint8Memory = new Uint8Array(wasm_exports.memory.buffer);
  Uint8Memory.subarray(ptr, ptr + typedArray.length).set(typedArray);
  return {ptr, length: typedArray.length};
};

export const getValue = (ptr: number, length: number) => {
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  const utf8decoder = new util.TextDecoder();
  return utf8decoder.decode(value);
};

export const setValueByBytes = (bytes: any) => {
  const typedArray = new Uint8Array(bytes);
  const ptr = wasm_exports._wasm_malloc(typedArray.length);
  const Uint8Memory = new Uint8Array(wasm_exports.memory.buffer);
  Uint8Memory.subarray(ptr, ptr + typedArray.length).set(typedArray);
  return {ptr, length: typedArray.length};
};

export const _get_timestamp = () => {
  return Date.now();
};

export const _gen_rand32_callback = (fn: number, addr: number) => {
  const buffer = crypto.randomBytes(32);
  const { ptr, length } = setValueByBytes(buffer);
  wasm_exports.call_gen_rand32_callback_fn(ptr, length, fn, addr);
  wasm_exports._wasm_free(ptr, length);
};
