const { actor_bin_instance } = require("../../index");

const print = (ptr, length) => {
  const value = actor_bin_instance.exports.memory.buffer.slice(ptr, ptr + length);
  let utf8decoder = new TextDecoder();
  console.log(utf8decoder.decode(value));
  // console.log(ptr, length);
};

module.exports = {
  print,
};
