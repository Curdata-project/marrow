import * as util from "util";
import * as crypto from "crypto";
import * as protobuf from "protobufjs";


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
  return {ptr, length: typedArray.length };
};

export const getValueByBytes = (ptr: number, length: number) => {
  const buffer = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  return buffer;
};

export const _get_timestamp = () => {
  return Date.now();
};

export const _gen_rand32_callback = (fn: number, addr: number) => {
  const root = protobuf.loadSync("common/proto/common.proto");
  const Sql = root.lookupType("Sql");
  const payload: any = {
    sql: "hello rust ~",
    params: []
  };
  const protomessage = Sql.create(payload);
  const buffer = Buffer.from("hello rust ~");
  // const buffer = Sql.encode(protomessage).finish();
  console.log(new Uint8Array(buffer));
  // const target = new Uint8Array(buffer).slice(0, 4);
  // const textdeocde = new util.TextDecoder();
  // console.log(textdeocde.decode(target));
  const { ptr, length } = setValueByBytes(buffer);
  wasm_exports.call_gen_rand32_callback_fn(ptr, length, fn, addr);
  wasm_exports._wasm_free(ptr, length);
};

export const _load_callback = () => {

};

export const _load_run = () => {

};
