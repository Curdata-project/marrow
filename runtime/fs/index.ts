import * as fs from "fs";
import * as util from "util";
import { getWasmExport } from "../storage";

export const _read_file_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const wasm_exports = getWasmExport();
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + path_length);
  const utf8decoder = new util.TextDecoder();
  const path = utf8decoder.decode(value);

  fs.readFile(path, (err, data) => {
    // console.log(fn, addr);
    wasm_exports.call_read_file_callback_fn(1, fn, addr);
  });
  return 0;
};
