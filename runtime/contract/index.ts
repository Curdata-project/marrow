import { getWasmExport } from "../storage";
import { log } from "../utils/log";

const contractList: any[] = [];

export const getContract = async (moduleName: string, ptr: number, length: number) => {
  const wasm_exports = getWasmExport(moduleName);
  const buffer = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  log().info(buffer, ptr, length, "from get contract");
  const { instance } = await WebAssembly.instantiate(buffer, {});
  contractList.push(instance);
  return contractList.length - 1;
};

export const runContract = (id: number, ptr: number, length: number) => {
  log().info(id, ptr, length, "begin run contract");
  const contract = contractList[id];
  log().info(contract, "the contract got when run contract");
  // todo: add env from ptr & length
  const result = contract.exports.main();
  return result;
};
