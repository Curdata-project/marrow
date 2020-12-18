const fs = require('fs');

const { print } = require("./tools/debug");
const { _read_file_callback } = require("./tools/fs");

let actor_bin_instance = null;
(async () => {
  let actor_bin_code = fs.readFileSync('target/wasm32-unknown-unknown/release/demo.wasm');

  let import_object = {
    wstd: {
      print,
      _read_file_callback,
    }
  };

  actor_bin_instance = (await WebAssembly.instantiate(actor_bin_code, import_object)).instance;

  actor_bin_instance.exports._entry();
})();

module.exports.actor_bin_instance = actor_bin_instance;
