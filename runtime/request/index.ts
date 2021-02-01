import axios from "axios";

import { getWasmExport } from "../storage";
import { getValue, setValue } from "../utils";

export const _request_callback = (moduleName: string) => {
  return function _request_callback (ptr: number, path_length: number, fn: number, addr: number) {
    const wasm_exports = getWasmExport();
    const value = getValue(moduleName, ptr, path_length);
    const arg = JSON.parse(value);
    axios({...arg}).then(result => {
      const { ptr, length } = setValue(moduleName, result.data);
      wasm_exports.call_request_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    }).catch(error => {
      const { ptr, length } = setValue(moduleName, error);
      wasm_exports.call_request_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    });
  };
};
