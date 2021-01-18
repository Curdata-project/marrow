import * as util from "util";
import { wasm_exports } from "../index";
import { log } from "../utils/log";

export const print = (ptr: number, length: number) => {
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  const utf8decoder = new util.TextDecoder();
  log().note(utf8decoder.decode(value));
  // console.log(ptr, length);
};
