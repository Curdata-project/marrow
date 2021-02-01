import * as util from "util";
import { getWasmExport } from "../storage";
import { log } from "../utils/log";

export const print = (wasmName: string) => {
  return function print (ptr: number, length: number) {
    const wasm_exports = getWasmExport(wasmName);
    log().star(ptr, length, "from js print");
    const value = wasm_exports.memory.buffer.slice(ptr, ptr + length);
    const utf8decoder = new util.TextDecoder();
    log().star(utf8decoder.decode(value));
  };
};
