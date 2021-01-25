import { IMessage } from "websocket";

import { isModuleExist, getWasmExport, isMethodExits, getCurMethod, changeCurWasm } from "../storage";
import { sendResponseSync } from "../notify";
import { log } from "../utils/log";
import { setValueByBytes } from "../utils/index";

export let requestCache: RequestCache[] = [];

export const handler = (message: IMessage) => {
  const requestData: any = JSON.parse(message.utf8Data);

  log().info("receive a new message 📧", requestData.index);
  const { index, type, module, name, args } = requestData;

  changeCurWasm(module);

  if (!index || !type || !name || !args) {
    return {
      code: 32601,
      message: "The sent json is not a valid request object",
      index,
    };
  }

  if (requestCache.length !== 0 && requestCache.includes(index)) {
    return {
      code: 32603,
      message: "The requested index already exists",
      index,
    };
  }

  if (!isModuleExist(module)) {
    return {
      code: 32602,
      message: "The module does not exist or is invalid",
      index,
    };
  }

  if (!isMethodExits(module, name)) {
    return {
      code: 32602,
      message: "The method does not exist or is invalid",
      index,
    };
  }

  const wasm_exports = getWasmExport(module);
  const curMethod = getCurMethod(module, name);

  if (curMethod.arguments.length !== args.length) {
    return {
      code: 32602,
      message: "The number of arguments does not match",
      index,
    };
  }

  const finalArgs = argsParse(index, args);

  wasm_exports[name](...finalArgs);

  const response = {
    code: 0,
    jsonrpc: "2.0",
    index,
    result: "",
  };
  sendResponseSync(response);
  requestCache.push(index);

};

// Function parameter checksum processing
const argsParse = (index: number, args: any[]): number[] => {
  let argsHandled: number[] = [index];

  if (args.length === 0) {
    return argsHandled;
  }

  for (let i = 0; i < args.length; i++) {
    const { type, value } = args[i];

    if (type === "i32") {
      argsHandled.push(value);
    }
    if (type === "bytes") {
      const base64Decode = Buffer.from(value, "base64");
      const { ptr, length } = setValueByBytes(base64Decode);
      argsHandled = argsHandled.concat([ptr, length]);
    }
  }

  return argsHandled;
};
