import { log } from "../utils/log";

const modulesList: ParseModuleList = [];

let curWasm: string;

export const getWasmExport = (name?: string): any => {
  const wasmName = name || curWasm;
  log().info(`get ${wasmName} module exports`);

  const curModule = modulesList.find(item => item.name === wasmName);
  return curModule.instance.exports;
};

export const addModuleInstance = (data: ParseModule) => {
  modulesList.push(data);
};

export const changeCurWasm = (name: string) => {
  if (isModuleExist(name)) {
    curWasm = name;
    log().info(`change current wasm to ${name}`);
  } else {
    log().warn(`change wasm to ${name} but failed`);
  }
};

export const isModuleExist = (name: string) => {
  return modulesList.map(item => item.name).includes(name);
};
