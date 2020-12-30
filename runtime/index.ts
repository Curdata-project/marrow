import * as fs from "fs";

import { print } from "./debug";
import { _read_file_callback } from "./fs";
import { _request_callback } from "./request";
import { _sql_run_callback, _sql_query_callback, _sql_operate_callback } from "./sqlite";
import {  } from "./notify";
import { _get_timestamp, _gen_rand32_callback } from "./utils";

import { startServer } from "./rpc/server";

export let wasm_exports: any;

const import_object = {
  wstd: {
    print,
    _read_file_callback,
    _request_callback,
    _sql_run_callback,
    _sql_query_callback,
    _get_timestamp,
    _sql_operate_callback,
    _gen_rand32_callback,
  }
};

export const initModule = async (path: string) => {
  const wasm = fs.readFileSync(path);
  
  const { instance, module } = await WebAssembly.instantiate(wasm, import_object);
  wasm_exports = instance.exports;
  return instance;
};

export const run = async (modules: Module[]) => {
  startServer(modules);
};
