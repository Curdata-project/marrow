import * as fs from "fs";

import { mw_rt } from "./notify/callback";

import { print } from "./debug";
import { _read_file_callback } from "./fs";
import { _request_callback } from "./request";
import { _sql_run_callback, _sql_query_callback, _sql_operate_callback } from "./sqlite";
import { _get_timestamp, _gen_rand32_callback, _load_callback, _load_run, _callback_number, setValueByBytes } from "./utils";

import { startServer, startTest } from "./rpc/server";
import { log } from "./utils/log";

import { getModuleMethods, changeCurWasm, getWasmExport, addModuleInstance } from "./storage";

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

let env: any = {};

export const initModule = async (module: Module) => {
  log().info(module.name, "module begin init");

  const import_object = importGenerate(module);

  const moduleExpose = getModuleMethods(module.name);
  

  try {
    const wasm = fs.readFileSync(module.path);
    const { instance } = await WebAssembly.instantiate(wasm, import_object);
    addModuleInstance({
      name: module.name,
      instance: instance,
    });
    changeCurWasm(module.name);

    envControl(instance, moduleExpose, module.name);
    
    instance.exports._entry();
  } catch (error) {
    log().error("module init error", error);
  }
};

const importGenerate = (module: Module) => {
  let import_object: any = {
    wstd,
    mw_rt,
  };
  if (module.deps.length === 0) {
    return import_object;
  } else {
    for (let i = 0; i < module.deps.length; i++) {
      import_object[module.deps[i]] = env[module.deps[i]];
    }
    return import_object;
  }
};

const envControl = (instance: WebAssembly.Instance, moduleExpose: any, moduleName: string) => {
  env[moduleName] = {};

  for (let i = 0; i < moduleExpose.length; i ++) {
    let finalArgs: any[] = ["index"];

    const argsDefined = moduleExpose[i].arguments;

    if (argsDefined.length !== 0) {
      for (let j = 0; j < argsDefined.length; j++) {
        if (argsDefined[j].type === "i32") {
          finalArgs.push("number");
        }
        if (argsDefined[j].type === "bytes") {
          finalArgs = finalArgs.concat(["ptr", "length"]);
        }
      }
    };

    env[moduleName][moduleExpose[i].name] = (...finalArgs: any) => {
      instance.exports[moduleExpose[i].name](...finalArgs);
    };

  }
};

export const test = async (modules: Module[]) => {
  startTest(modules);
};

export const run = async (modules: Module[]) => {
  wasm_modules_amount = modules.length;
  startServer(modules);
};
