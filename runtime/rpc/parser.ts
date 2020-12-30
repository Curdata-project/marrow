import * as protobuf from "protobufjs";
import { initModule } from "../index";

// Todo: type defined


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
    
  }

  return outputModules;
};