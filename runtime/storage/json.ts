
let methodsList: any = {};

export const initMethodsList = (list: any) => {
  methodsList = list;
};

export const isMethodExits = (moduleName: string, methodName: string) => {
  const curModule = methodsList[moduleName];
  return curModule.map((item: any) => item.name).includes(methodName);
};

export const getModuleMethods = (moduleName: string) => {
  return methodsList[moduleName];
};

export const getCurMethod = (moduleName: string, methodName: string) => {
  if (isMethodExits(moduleName, methodName)) {
    return methodsList[moduleName].find((item: any) => item.name === methodName);
  }
};
