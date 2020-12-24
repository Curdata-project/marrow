import { server, connection } from "websocket";
import * as http from "http";

import { hander } from "./hander";

export let socket: connection;

export const startServer = (modules: Modules) => {

  const httpServer = http.createServer();

  const wsServer = new server({
    httpServer: httpServer,
    autoAcceptConnections: false,
  });
  
  wsServer.on("request", request => {
    console.log("request");

    const connect = request.accept('echo-protocol', request.origin);
    socket = connect;

    connect.on("message", async (message) => {
      console.log(message);
      
      try {
        await hander(message, modules);
      } catch (error) {
        
      }
      
    })

    connect.on("close", () => {
      console.log("close");
    })

  });

  httpServer.listen(3003, () => { console.log("running") });
  
  
};

