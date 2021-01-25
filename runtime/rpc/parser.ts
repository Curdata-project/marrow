import { promises as fs } from "fs";
import * as events from "events";

import { initModule } from "../index";
import { log } from "../utils/log";

export const event = new events.EventEmitter();

export const loadJson = async (folder: string) => {
  try {
    const filesName = await fs.readdir(folder);
    if (filesName.length === 0) {
      log().warn("empty folder");
      return;
    }
    const result: any = {};
    for (let i = 0; i < filesName.length; i++) {
      const buffer = await fs.readFile(`target/abi/${filesName[i]}`);
      const data: Method[] = JSON.parse(buffer.toString());
      result[filesName[i].slice(0, -5)] = data;
    }
    return result;
  } catch (error) {
    log().error("folder not found");
    return {};
  }
};

export const wasmParser = async (modules: Modules) => {

  const ordered = depsParser(modules);

  event.on("next_wasm_init", async (index: number) => {
    await initModule(ordered[index]);
  });

  await initModule(ordered[0]);
};

export const depsParser = (modules: Modules): Modules => {
  const used = new Set;
  const result: Modules = [];
  let nameContainer: any[];
  let moduleContainer: any[];
  let length;

  do {
    length = modules.length;
    nameContainer = [];
    moduleContainer = [];
    modules = modules.filter(item => {
      if (!item.deps.every(Set.prototype.has, used)) {
        return true;
      }
      nameContainer.push(item.name);
      moduleContainer.push(item);
    });
    result.push(...moduleContainer);
    nameContainer.forEach(Set.prototype.add, used);
  } while (modules.length && modules.length !== length);

  result.push(...modules);
  return result;
};
