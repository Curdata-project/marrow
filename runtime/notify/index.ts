import { socket } from "../rpc/server";

export const sendResponseSync = ({ code, index, result, jsonrpc = "2.0" }: RPCResponse) => {
  const response = {
    jsonrpc,
    code,
    index,
    result,
  };
  socket.send(JSON.stringify(response));
};
