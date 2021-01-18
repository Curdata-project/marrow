import { IMessage } from "websocket";

import { modulesNameIndex, modulesList } from "./server";
import { sendResponseSync } from "../notify";
import { log } from "../utils/log";

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

  const argsBuffer = Buffer.from(args, "base64");
  const result = curModule.instance.exports[name](argsBuffer);
  log().info(result, "result");
  const response = {
    code: 0,
    jsonrpc: "2.0",
    index,
    result: result,
  };
  sendResponseSync(response);
  requestCache.push(index);

};
