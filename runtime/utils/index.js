const util = require("util");

const { actor_bin_instance } = require("../index");

const { memory, _malloc } = actor_bin_instance.exports;

const setValue = (value) => {
  const textEncoder = new util.TextEncoder();
  const typedArray = textEncoder.encode(value);
  const ptr = _malloc(typedArray.length);
  const Uint8Memory = new Uint8Array(memory.buffer);
  Uint8Memory.subarray(ptr, ptr + typedArray.length).set(typedArray);
  return {ptr, lenth: typedArray.length};
};

const getValue = (ptr, length) => {
  const value = memory.buffer.slice(ptr, ptr + length);
  const utf8decoder = new util.TextDecoder();
  return utf8decoder.decode(value);
};

module.exports = {
  setValue,
  getValue,
};
