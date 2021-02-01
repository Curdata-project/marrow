import * as fs from "fs";

import { mw_rt } from "./notify/callback";

import { print } from "./debug";
import { _read_file_callback } from "./fs";
import { _request_callback } from "./request";
import { _sql_run_callback, _sql_query_callback, _sql_operate_callback } from "./sqlite";
import { _get_timestamp, _gen_rand32_callback, _load_callback, _load_run, _callback_number } from "./utils";

import { startServer, startTest } from "./rpc/server";
import { log } from "./utils/log";

import { getModuleMethods, getWasmExport, addModuleInstance, setIndex } from "./storage";

export let wasm_modules_amount: number;

const wstd = (moduleName: string) => {
  return {
    print: print(moduleName),
    _read_file_callback,
    _request_callback: _request_callback(moduleName),
    _sql_run_callback: _sql_run_callback(moduleName),
    _sql_query_callback: _sql_query_callback(moduleName),
    _sql_operate_callback: _sql_operate_callback(moduleName),
    _gen_rand32_callback,
    _load_callback: _load_callback(moduleName),
    _load_run: _load_run(),
    _callback_number,
  };
};

export const initModule = async (module: Module) => {
  log().info(module.name, "module begin init");

  const import_object = importGenerate(module);

  try {
    const wasm = fs.readFileSync(module.path);
    const { instance } = await WebAssembly.instantiate(wasm, import_object);
    addModuleInstance({
      name: module.name,
      instance: instance,
    });

    instance.exports._entry();
  } catch (error) {
    log().error("module init error", error);
  }
};

const importGenerate = (module: Module) => {
  const import_object: any = {
    wstd: wstd(module.name),
    mw_rt,
  };

  if (module.deps.length === 0) {
    return import_object;
  } else {
    for (const dep of module.deps) {

      import_object[dep] = {};

      const moduleExpose = getModuleMethods(dep);

      for (const expose of moduleExpose) {
        let finalArgs: any[] = ["index"];

        const argsDefined = expose.arguments;

        if (argsDefined.length !== 0) {
          for (const arg of argsDefined) {
            if (arg.type === "i32") {
              finalArgs.push("number");
            }
            if (arg.type === "bytes") {
              finalArgs = finalArgs.concat(["ptr", "length"]);
            }
          }
        }

        const ptrIndex = finalArgs.findIndex(item => item === "ptr");

        // from: module.name
        // to: dep

        import_object[dep][expose.name] = (...finalArgs: any) => {
          const args = finalArgs;

          const cb = args.pop();
          const user_data = args.pop();
          const index = cb + user_data;

          args.unshift(index);

          setIndex(index, { cb, user_data, moduleName: module.name, funcName: expose.name });

          const from_exports = getWasmExport(dep);
          const to_exports = getWasmExport(module.name);

          const ptrValue = finalArgs[ptrIndex];
          const lengthValue = finalArgs[ptrIndex + 1];
          const memory = to_exports.memory.buffer.slice(ptrValue, ptrValue + lengthValue);
          const typedArray = new Uint8Array(memory);

          const ptr = from_exports._wasm_malloc(typedArray.length);
          const Uint8Memory = new Uint8Array(from_exports.memory.buffer);
          Uint8Memory.subarray(ptr, ptr + typedArray.length).set(typedArray);

          finalArgs[ptrIndex] = ptr;
          finalArgs[ptrIndex + 1] = typedArray.length;
          from_exports[expose.name](...args);
        };

      }


    }
    return import_object;
  }
};

export const test = async (modules: Module[]) => {
  startTest(modules);
};

export const run = async (modules: Module[]) => {
  wasm_modules_amount = modules.length;
  startServer(modules);
};
