import { server, connection } from "websocket";
import * as http from "http";

import { handler } from "./handler";
import { loadJson, wasmParser } from "./parser";

export let socket: connection;

export let methodsList: Method[];
export let modulesList: ParseModuleList;

export let methodsNameIndex: string[];
export let modulesNameIndex: string[];

// For Make Test
export const startTest = async (modules: Modules) => {
  // load .json
  try {
    console.log("begin parser json");
    const result = await loadJson("target/abi/");
    methodsList = result;
    methodsNameIndex = result.map(item => item.name);
    console.log(result, "load json result");
    console.log("json files parser success 🌟");
  } catch (error) {
    console.log("json parser fail", error);
    return;
  }

  // load .wasm
  try {
    console.log("begin parser modules");
    const result = await wasmParser(modules);
    modulesList = result;
    modulesNameIndex = result.map(item => item.name);
    console.log(result, "load modules result");
    console.log("wasm files parser success 🦀️");
  } catch (error) {
    console.log("wasm parser fail", error);
    return;
  }
};

export const startServer = async (modules: Modules) => {

  // load .json
  try {
    console.log("begin parser json");
    const result = await loadJson("target/abi/");
    methodsList = result;
    methodsNameIndex = result.map(item => item.name);
    console.log(result, "load json result");
    console.log("json files parser success 🌟");
  } catch (error) {
    console.log("json parser fail", error);
    return;
  }

  // load .wasm
  try {
    console.log("begin parser modules");
    const result = await wasmParser(modules);
    modulesList = result;
    modulesNameIndex = result.map(item => item.name);
    console.log(result, "load modules result");
    console.log("wasm files parser success 🦀️");
  } catch (error) {
    console.log("wasm parser fail", error);
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

      const error = handler(message);
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

