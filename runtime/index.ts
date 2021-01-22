import * as fs from "fs";

import { mw_rt } from "./notify/callback";

import { print } from "./debug";
import { _read_file_callback } from "./fs";
import { _request_callback } from "./request";
import { _sql_run_callback, _sql_query_callback, _sql_operate_callback } from "./sqlite";
import { _get_timestamp, _gen_rand32_callback, _load_callback, _load_run, _callback_number } from "./utils";

import { startServer, startTest, methodsList } from "./rpc/server";
import { log } from "./utils/log";

import { wasm_exports, changeExports } from "./rpc/handler";

export let wasm_modules_amount: number;

const wstd = {
  print,
  _read_file_callback,
  _request_callback,
  _sql_run_callback,
  _sql_query_callback,
  _sql_operate_callback,
  _gen_rand32_callback,
  _load_callback,
  _load_run,
  _callback_number,
};

const env: any = {};

export const initModule = async (module: Module) => {
  log().info(module.name, "module begin init");
  const import_object: any = {
    wstd,
    mw_rt,
  };
  if (module.deps.length !== 0) {
    for (let i = 0; i < module.deps.length; i++) {
      import_object[module.deps[i]] = env[module.deps[i]];
    }
  }
  try {
    const wasm = fs.readFileSync(module.path);
    const { instance } = await WebAssembly.instantiate(wasm, import_object);
    changeExports(instance.exports);
    const curModuleExpose = methodsList[module.name].map((item: Method) => item.name);
    env[module.name] = {};
    for (let i = 0; i < curModuleExpose.length; i ++) {
      const curWasmExport = wasm_exports[curModuleExpose[i]];
      env[module.name][curModuleExpose[i]] = () => {
        return function (index: number, ptr: number, length: number) {
          curWasmExport(index, ptr, length);
        };
      };
    }
    instance.exports._entry();
    return instance;
  } catch (error) {
    log().error("module init error", error);
  }
};

export const test = async (modules: Module[]) => {
  startTest(modules);
};

export const run = async (modules: Module[]) => {
  wasm_modules_amount = modules.length;
  startServer(modules);
};
