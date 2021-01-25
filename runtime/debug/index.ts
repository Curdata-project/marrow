import * as util from "util";
import { getWasmExport } from "../storage";
import { log } from "../utils/log";

export const print = (ptr: number, length: number) => {
  const wasm_exports = getWasmExport();
  log().note(ptr,length,"js print");
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  const utf8decoder = new util.TextDecoder();
  log().note(utf8decoder.decode(value));
  // log().note(ptr, length, value);
};
