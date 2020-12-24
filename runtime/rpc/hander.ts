import { IMessage } from "websocket";

export let moduleCache: CacheModule[];

export const hander = (message: IMessage, modules: Modules) => {
  const data = JSON.parse(message.utf8Data);
  const { index, type, name, args } = data;
  if (!index|| !type || !name || !args) {
    return {
      code: 32601,
      message: "发送的json不是一个有效的请求对象",
    };
  };

  if (moduleCache.map(item => item.index).includes(index)) {
    return {
      code: 32603,
      message: "请求的 index 已存在",
    };
  };

  const targetModule = modules.find(module => {
    const methodList = module.expose.map(item => item.name);
    return methodList.includes(name);
  });
  if (!targetModule) {
    return {
      code: 32601,
      message: "该方法不存在或无效",
    };
  };

  const targetMethod = targetModule.expose.find(item => item.name === name);

  if (args.length !== targetMethod.args.length) {
    return {
      code: 32602,
      message: "无效的方法参数",
    };
  };

  moduleCache.push({
    index: index,
    targetMethod,
  });

  

};
