import { IMessage } from "websocket";

import { methodsList, modulesList, methodsNameIndex, modulesNameIndex } from "./server";
import { sendResponseSync } from "../notify";

export let requestCache: RequestCache[] = [];

export const handler = (message: IMessage) => {
  const requestData: RPCRequest = JSON.parse(message.utf8Data);

  console.log("receive a new message 📧", requestData);
  const { index, type, module, name, args } = requestData;

  if (!index || !type || !name || !args) {
    return {
      code: 32601,
      message: "The sent json is not a valid request object",
      index,
    };
  }

  if (requestCache.length !== 0 && requestCache.map(item => item.index).includes(index)) {
    return {
      code: 32603,
      message: "The requested index already exists",
      index,
    };
  }

  if (!methodsNameIndex.includes(name)) {
    return {
      code: 32601,
      message: "The method does not exist or is invalid",
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
  const curMethod = methodsList.find(method => method.name = name);
  const curModule = modulesList.find(item => item.name = module);

  if (curMethod.ty === "sync") {
    const result = curModule.instance.exports[name](...args);
    const response = {
      code: 0,
      jsonrpc: "2.0",
      index,
      result,
    };
    sendResponseSync(response);
  }

};
