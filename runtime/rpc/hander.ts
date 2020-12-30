import { IMessage } from "websocket";
import { setValueByBytes } from "../utils";

import { wasm_exports } from "../index";

export let moduleCache: RequestCache[] = [];

export const hander = (message: IMessage, modules: any) => {
  const data: RPCRequest = JSON.parse(message.utf8Data);

  console.log("receive a new message 📧", data);
  const { index, type, module, name, args } = data;

  if (!index || !type || !name || !args) {
    return {
      code: 32601,
      message: "The sent json is not a valid request object",
      index,
    };
  }

  if (moduleCache.length !== 0 && moduleCache.map(item => item.index).includes(index)) {
    return {
      code: 32603,
      message: "The requested index already exists",
      index,
    };
  }

  const method = modules[module][name];

  if (!method) {
    return {
      code: 32601,
      message: "The method does not exist or is invalid",
      index,
    };
  }

  if (method.args.length !== args.length) {
    return {
      code: 32602,
      message: `Invalid method parameter, ${method.args.length} defined but ${args.length} received`,
      index,
    };
  }

  moduleCache.push({
    proto: method.return,
    index,
  });

  console.log("当前的 module cache", moduleCache);
  // Todo: 区别 callback

  
  
    modules[module].instance.exports[name](index, ...args);
  

  if (method.args[0].type === "bytes") {
    const { ptr, length } = setValueByBytes(args);
    modules[module].instance.exports[name](index, ptr, length);
    wasm_exports._wasm_free(ptr, length);
  }

};
