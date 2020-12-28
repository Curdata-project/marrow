import { socket } from "../rpc/server";
import { moduleCache } from "../rpc/hander";
import { getValue } from "../utils";

export const _callback_ptr_size = (dex: number, ptr: number, size: number) => {
  const value = getValue(ptr, size);
  const curModule = moduleCache.find(item => item.index === dex);
  const valueDecoded = curModule.module.return.decode(value);
  const response: Response = {
    jsonrpc: "2.0",
    result: valueDecoded,
    code: 0,
    index: dex,
  };
  socket.send(JSON.stringify(response));
};

export const _callback_number = (index: number, number: number) => {
  const response: Response = {
    jsonrpc: "2.0",
    result: number,
    index: index,
    code: 0,
  };
  socket.send(JSON.stringify(response));
};
