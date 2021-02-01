import { log } from "../utils/log";
import { getIndex, getWasmExport } from "../storage";
import { sendResponseSync } from "./index";

const __callback_u32 = (index: number, result: number) => {

};

const __callback_i32 = (index: number, result: number) => {
  if (index > 10) {
    log().info(index, result, "from notify i32");
    const { cb, user_data, moduleName, funcName } = getIndex(index);
    const wasm_exports = getWasmExport(moduleName);
    log().info(`call_${funcName}`, result, user_data, cb);
    wasm_exports[`call_${funcName}`](result, user_data, cb);
  } else {
    log().info(index, result, "from notify i32");
    const response = {
      code: 0,
      jsonrpc: "2.0",
      index,
      result: "",
    };
    sendResponseSync(response);
  }
};

const __callback_u64 = (index: number, result: number) => {

};

const __callback_i64 = (index: number, result: number) => {

};

const __callback_isize = (index: number, result: number) => {

};

const __callback_usize = (index: number, result: number) => {

};

const __callback_bytes = (index: number, ptr: number, length: number) => {
  // todo: use protobuf decode result
  log().info(index, ptr, length, "from notify bytes");
};

const __callback_null = (index: number) => {

};

export const mw_rt = {
  __callback_u32,
  __callback_i32,
  __callback_i64,
  __callback_u64,
  __callback_isize,
  __callback_usize,
  __callback_bytes,
  __callback_null,
};
