import { socket } from "../rpc/server";
import { moduleCache } from "../rpc/hander";
import { getValue } from "../utils";

export const _callback_ptr_size = (dex: number, ptr: number, size: number) => {
  console.log("send a string type response", dex, ptr, size);
  let result: any;
  if (size === 0) {
    result = "null";
  } else {
    const value = getValue(ptr, size);
    const proto = moduleCache.find(item => item.index === dex).proto;
    const valueDecoded = proto.value.decode(value);
    result = valueDecoded;
  }
  const response: Response = {
    jsonrpc: "2.0",
    result: result,
    code: 0,
    index: dex,
  };
  socket.send(JSON.stringify(response));
};

export const _callback_number = (index: number, number: number) => {
  console.log("send a number type response");
  const response: Response = {
    jsonrpc: "2.0",
    result: number,
    index: index,
    code: 0,
  };
  socket.send(JSON.stringify(response));
};
