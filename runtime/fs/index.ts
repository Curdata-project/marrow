import * as fs from "fs";
import * as util from "util";
import { wasm_exports } from "../index";

export const _read_file_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + path_length);
  const utf8decoder = new util.TextDecoder();
  const path = utf8decoder.decode(value);

  fs.readFile(path, (err, data) => {
    console.log(fn, addr);
    wasm_exports.call_read_file_callback_fn(fn, addr, 1);
  });
  return 0;
};
