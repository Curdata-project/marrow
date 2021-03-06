import * as fs from "fs";

import { print } from "./debug";
import { _read_file_callback } from "./fs";
import { _request_callback } from "./request";
import { _sql_run_callback, _sql_query_callback } from "./sqlite";

type Module = {
  name: string,
  path: string,
};

export let wasm_exports: any;

const import_object = {
  wstd: {
    print,
    _read_file_callback,
    _request_callback,
    _sql_run_callback,
    _sql_query_callback,
  }
};

export const run = async (modules: Module[]) => {
  for (let i = 0; i < modules.length; i++) {
    const wasm = fs.readFileSync(modules[i].path);
    // @ts-ignore
    const { instance, module } = await WebAssembly.instantiate(wasm, import_object);
    wasm_exports = instance.exports;
    console.log(wasm_exports);
    wasm_exports._entry();
  }
};

// run(modules);
