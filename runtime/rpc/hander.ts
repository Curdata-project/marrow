import { IMessage } from "websocket";
import { setValueByBytes } from "../utils";

import { wasm_exports } from "../index";

export let moduleCache: CacheModule[] = [];

export const hander = (message: IMessage, modules: any) => {
  const data: Request = JSON.parse(message.utf8Data);

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

  if (method.args.length === 0) {
    modules[module].instance.exports[name](index);
  }

  if (method.args[0].type === "number") {
    modules[module].instance.exports[name](index, ...args);
  }

  if (method.args[0].type === "bytes") {
    const { ptr, length } = setValueByBytes(args);
    modules[module].instance.exports[name](index, ptr, length);
    wasm_exports._wasm_free(ptr, length);
  }

  if (method.args[0].type === "proto") {
    const message = method.args[0].message;
    const argsError = message.verify(args[0]);
    if (argsError) {
      console.log(argsError, "protobuf verify error");
      return {
        code: 32602,
        message: "protobuf verify error",
        index,
      };
    }
    const msg = message.create(args[0]);
    const buffer = message.encode(msg).finish();
    console.log(buffer, "this is protobuf encode output");
    const { ptr, length } = setValueByBytes(buffer);
    console.log(`run wasm ${name} function and index=${index} ptr=${ptr} length=${length}`);
    modules[module].instance.exports[name](index, ptr, length);
    wasm_exports._wasm_free(ptr, length);
  }

};
