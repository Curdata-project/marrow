import * as protobuf from "protobufjs";
import { initModule } from "../index";

// Todo: type defined

/* Parser Result Demo
*
*  const parserResultDemo = {
*    keystore: {
*      instance: "instance", // Compiled wasm
*      list_accounts: {
*        type: "callback",
*        args: [ { name: 'page', type: 'number' } || { name: 'item', type: 'bytes' } || { name: 'order', type: 'proto', message: message } ],
*        return: {
*          type: proto || number,
*          value: message
*        }
*      }
*    }
*  }
*/

export const parser = async (modules: any) => {
  const outputModules: any = {};

  // Todo: Dependent collection and sorting

  for (const item in modules) {
    const curModule = modules[item];
    outputModules[item] = {};

    const instance = await initModule(curModule.path);
    outputModules[item].instance = instance;

    // init wasm module
    instance.exports._entry();
    console.log("entry");

    const methodsList = curModule.expose;
    for (let j = 0; j < methodsList.length; j++) {
      const curMethod = methodsList[j];
      outputModules[item][curMethod.name] = {};
      outputModules[item][curMethod.name].type = curMethod.type;

      if (curMethod.args.length === 0) {
        outputModules[item][curMethod.name].args = curMethod.args;
      } else {
        if (curMethod.args[0].type === "number" || curMethod.args[0].type === "bytes") {
          outputModules[item][curMethod.name].args = curMethod.args;
        }

        if (curMethod.args[0].type === "proto") {
          const protoPath = curMethod.args[0].attr.proto;
          const protoMessage = curMethod.args[0].attr.message;

          const protoRoot = protobuf.loadSync(protoPath);
          const message = protoRoot.lookupType(protoMessage);

          outputModules[item][curMethod.name].args = [{
            type: "proto",
            name: curMethod.args[0].name,
            message: message,
          }];
        }
      }

      outputModules[item][curMethod.name].return = {};

      if (curMethod.return.type === "number") {
        outputModules[item][curMethod.name].return.type = "number";
      }

      if (curMethod.return.type === "proto") {
        outputModules[item][curMethod.name].return.type = "proto";

        const protoPath = curMethod.return.proto;
        const protoMessage = curMethod.return.message;

        const protoRoot = protobuf.loadSync(protoPath);
        const message = protoRoot.lookupType(protoMessage);
        outputModules[item][curMethod.name].return.value = message;
      }

    }
  }

  return outputModules;
};