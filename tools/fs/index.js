const fs = require("fs");
const { actor_bin_instance } = require("../../index");

const _read_file_callback = (fn, addr, ptr, path_length) => {
  const value = actor_bin_instance.exports.memory.buffer.slice(ptr, ptr + path_length);
  let utf8decoder = new TextDecoder();
  path = utf8decoder.decode(value);

  fs.readFile(path, (err, data) => {
    console.log(fn, addr);
    actor_bin_instance.exports.call_read_file_callback_fn(fn, addr, 1)
  });
  return 0;
};

module.exports = {
  _read_file_callback,
};
