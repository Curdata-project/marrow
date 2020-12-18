const fs = require("fs/promises");

const { print } = require("./debug");
const { _read_file_callback } = require("./fs");
const { _request_callback } = require("./request");
const { _sqlite_callback } = require("./sqlite");

let actor_bin_instance = null;

let import_object = {
  wstd: {
    print,
    _read_file_callback,
    _request_callback,
    _sqlite_callback,
  }
};

const run = async (modules) => {
  for (let item in modules) {
    try {
      const actor_bin_code = await fs.readFile(item.path);
      actor_bin_instance = (await WebAssembly.instantiate(actor_bin_code, import_object)).instance;
      actor_bin_instance.exports._entry();
    } catch (error) {
      console.log(error);
    }
  }
};

module.exports = {
  actor_bin_instance,
  run,
};
