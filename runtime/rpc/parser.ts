import { promises as fs } from "fs";
import { initModule } from "../index";

export const loadJson = async (folder: string): Promise<Method[]> => {
  try {
    const filesName = await fs.readdir(folder);
    if (filesName.length === 0) {
      console.log("empty folder");
      return;
    }
    let result: Method[] = [];
    for (let i = 0; i < filesName.length; i++) {
      const buffer = await fs.readFile(`target/abi/${filesName[i]}`);
      const data: Method[] = JSON.parse(buffer.toString());
      const output = result.concat(data);
      result = output;
    }
    return result;
  } catch (error) {
    console.log("folder not found");
    return [];
  }
};

export const wasmParser = async (modules: Modules): Promise<ParseModuleList> => {
  const parseModules: ParseModuleList = [];

  // Todo: Dependent collection and sorting

  for (let i = 0; i < modules.length; i++) {
    const instance = await initModule(modules[i].path);
    parseModules.push({
      name: modules[i].name,
      instance,
    });
    instance.exports._entry();
  }

  return parseModules;
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