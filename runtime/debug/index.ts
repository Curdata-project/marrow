import * as util from "util";
import { wasm_exports } from "../index";

export const print = (ptr: number, length: number) => {
  const value = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  let utf8decoder = new util.TextDecoder();
  console.log(utf8decoder.decode(value));
  // console.log(ptr, length);
};
