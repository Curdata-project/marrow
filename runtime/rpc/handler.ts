import { IMessage } from "websocket";

import { modulesNameIndex, modulesList, methodsList } from "./server";
import { sendResponseSync } from "../notify";
import { log } from "../utils/log";
import { setValueByBytes } from "../utils/index";
import { wasm_exports } from "../index";
import { common } from "protobufjs";

export let requestCache: RequestCache[] = [];

export const handler = (message: IMessage) => {
  const requestData: any = JSON.parse(message.utf8Data);

  log().info("receive a new message 📧", requestData.index);
  const { index, type, module, name, args } = requestData;

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

  if (!modulesNameIndex.includes(module)) {
    return {
      code: 32602,
      message: "The module of method does not exist or is invalid",
      index,
    };
  }

  // get module and method for request
  const curModule = modulesList.find(item => item.name = module);
  const curMethod = methodsList[module].find((item: any) => item.name === name)

  if (!curMethod) {
    return {
      code: 32602,
      message: "The method does not exist or is invalid",
      index,
    };
  }

  if (curMethod.arguments.length !== args.length) {
    return {
      code: 32602,
      message: "The number of arguments does not match",
      index,
    };
  }

  const finalArgs = argsParse(index, args);
  
  curModule.instance.exports[name](...finalArgs);
  
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
}
