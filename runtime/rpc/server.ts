import { server, connection } from "websocket";
import * as http from "http";

import { hander } from "./hander";
import { parser } from "./parser";

export let socket: connection;

export const startServer = async (modules: Modules) => {

  let modulesResolved: any;

  try {
    console.log("begin parser modules");
    const result = await parser(modules);
    modulesResolved = result;
    console.log(result, "编译结果");
    console.log("module parser success 🌟");
  } catch (error) {
    console.log("module parser fail", error);
    return;
  }

  const httpServer = http.createServer();

  const wsServer = new server({
    httpServer: httpServer,
    autoAcceptConnections: false,
  });

  wsServer.on("request", request => {
    console.log("a new connect request 🔗");

    const connect = request.accept("echo-protocol", request.origin);
    socket = connect;

    connect.on("message", async (message) => {

      const error = hander(message, modulesResolved);
      if (error) {
        const response: RPCResponse = {
          jsonrpc: "2.0",
          index: error.index,
          code: error.code,
          result: error.message,
        };
        connect.send(JSON.stringify(response));
      }

    });

    connect.on("close", () => {
      console.log("close");
    });

  });

  httpServer.listen(3003, () => { console.log("server is running on 3003 🚀"); });
};

