import axios from "axios";

import { wasm_exports } from "../index";
import { getValue, setValue } from "../utils";

export const _request_callback = (ptr: number, path_length: number, fn: any, addr: any) => {
  const value = getValue(ptr, path_length);
  const arg = JSON.parse(value);
  axios({...arg}).then(result => {
    const { ptr, length } = setValue(result.data);
    wasm_exports.call_request_callback_fn(ptr, length, fn, addr);
  }).catch(error => {
    const { ptr, length } = setValue(error);
    wasm_exports.call_request_callback_fn(ptr, length, fn, addr);
  })
};
