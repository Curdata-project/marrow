import { server, connection } from "websocket";
import * as http from "http";

import { handler } from "./handler";
import { loadJson, wasmParser } from "./parser";
import { log } from "../utils/log";
import { initMethodsList } from "../storage";

export let socket: connection;

// For Make Test
export const startTest = async (modules: Modules) => {
  try {
    log().info("begin parser json");
    const result = await loadJson("target/abi/");
    initMethodsList(result);
    log().success("json files parser success 🌟");
  } catch (error) {
    log().error("json parser fail", error);
    return;
  }

  try {
    await wasmParser(modules);
    log().success("wasm files parser success 🦀️");
  } catch (error) {
    log().error("wasm parser fail", error);
    return;
  }
};

export const startServer = async (modules: Modules) => {

  // load .json
  try {
    const result = await loadJson("target/abi/");
    initMethodsList(result);
    log().success("json files parser success 🌟");
  } catch (error) {
    log().error("json parser fail", error);
    return;
  }

  // load .wasm
  try {
    await wasmParser(modules);
    log().success("wasm files parser success 🦀️");
  } catch (error) {
    log().error("wasm parser fail", error);
    return;
  }

  const httpServer = http.createServer();

  const wsServer = new server({
    httpServer: httpServer,
    autoAcceptConnections: false,
  });

  wsServer.on("request", request => {
    log().info("a new connect request 🔗");

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

    connect.on("close", (reasonCode, description) => {
      log().info("close", reasonCode, description);
    });

  });

  httpServer.listen(3004, () => { log().success("server is running on 3003 🚀"); });
};

