import * as util from "util";

import { wasm_modules_amount } from "../index";
import { log } from "../utils/log";
import { event } from "../rpc/parser";
import { getContract, runContract } from "../contract";
import { getWasmExport } from "../storage";

export const setValue = (moduleName: string, value: string) => {
  const wasm_exports = getWasmExport(moduleName);
  const textEncoder = new util.TextEncoder();
  const typedArray = textEncoder.encode(value);
  const ptr = wasm_exports._wasm_malloc(typedArray.length);
  const Uint8Memory = new Uint8Array(wasm_exports.memory.buffer);
  Uint8Memory.subarray(ptr, ptr + typedArray.length).set(typedArray);
  return {ptr, length: typedArray.length};
};

export const getValue = (moduleName: string, ptr: number, length: number) => {
  const wasm_exports = getWasmExport(moduleName);
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  const utf8decoder = new util.TextDecoder();
  return utf8decoder.decode(value);
};

export const setValueByBytes = (moduleName: string, bytes: any) => {
  const wasm_exports = getWasmExport(moduleName);
  const typedArray = new Uint8Array(bytes);
  const ptr = wasm_exports._wasm_malloc(typedArray.length);
  const Uint8Memory = new Uint8Array(wasm_exports.memory.buffer);
  Uint8Memory.subarray(ptr, ptr + typedArray.length).set(typedArray);
  return {ptr, length: typedArray.length };
};

export const getValueByBytes = (moduleName: string, ptr: number, length: number) => {
  const wasm_exports = getWasmExport(moduleName);
  const buffer = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  return buffer;
};

export const _get_timestamp = () => {
  return Date.now();
};

export const _gen_rand32_callback = (fn: number, addr: number) => {};

export const _load_callback = (moduleName: string) => {
  return async function _load_callback (ptr: number, size: number, cb: number, user_data: number) {
    const wasm_exports = getWasmExport(moduleName);
    log().info(ptr, size, cb, user_data, "from load callback");
    const index = await getContract(moduleName, ptr, size);
    wasm_exports.call_loader_callback_fn(index, cb, user_data);
  };
};

export const _load_run = () => {
  return function _load_run (index: number, ptr: number, size: number) {
    const result = runContract(index, ptr, size);
    return result;
  };
};

let wasm_init_next = 1;

// When the previous wasm init is completed, this method will
// be notified to call the next wasm init
export const _callback_number = (index: number, num: number) => {
  if (wasm_init_next >= wasm_modules_amount) {
    log().info("wasm modules init complate");
    return;
  } else {
    log().info(`wasm entry callback, begin init module ${wasm_init_next}`);
    event.emit("next_wasm_init", wasm_init_next);
    wasm_init_next++;
  }
};
