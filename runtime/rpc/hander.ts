import { IMessage } from "websocket";
import { setValueByBytes } from "../utils";

export let moduleCache: CacheModule[] = [];

export const hander = (message: IMessage, modules: any) => {
  const data: Request = JSON.parse(message.utf8Data);
  const { index, type, module, name, args } = data;

  if (!index || !type || !name || !args) {
    return {
      code: 32601,
      message: "发送的json不是一个有效的请求对象",
      index,
    };
  }
  console.log("当前的 module cache", moduleCache);
  if (moduleCache.length !== 0 && moduleCache.map(item => item.index).includes(index)) {
    return {
      code: 32603,
      message: "请求的 index 已存在",
      index,
    };
  }

  const method = modules[module][name];
  if (!method) {
    return {
      code: 32601,
      message: "该方法不存在或无效",
      index,
    };
  }

  if (method.args.length !== args.length) {
    return {
      code: 32602,
      message: "无效的方法参数",
      index,
    };
  }

  moduleCache.push({
    index,
    module,
    name,
  });

  // Todo: 调用对应的模块方法
  // Todo: 区别 callback

  if (method.args.length === 0) {
    modules[module].instance.exports[name]();
  }

  if (method.args[0].type === "number") {
    modules[module].instance.exports[name](...args);
  }

  if (method.args[0].type === "bytes") {
    const { ptr, length } = setValueByBytes(args);
    modules[module].instance.exports[name](ptr, length);
  }

  if (method.args[0].type === "proto") {
    const message = method.args[0].message;
    const argsError = message.verify(args);
    if (argsError) {
      return {
        code: 32602,
        message: "无效的方法参数",
        index,
      };
    }

    const buffer = message.encode(args).finish();
    const { ptr, length } = setValueByBytes(buffer);
    modules[module].instance.exports[name](ptr, length);
  }

};
