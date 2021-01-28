import { getWasmExport } from "../storage";
import { log } from "../utils/log";

const contractList: any[] = [];

export const getContract = async (moduleName: string, ptr: number, length: number) => {
  log().info("开始获取合约");
  const wasm_exports = getWasmExport(moduleName);
  const buffer = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  console.log(buffer, ptr, length, "from get contract");
  const { instance } = await WebAssembly.instantiate(buffer, {});
  contractList.push(instance);
  return contractList.length - 1;
};

export const runContract = (id: number, ptr: number, length: number) => {
  log().info(id, ptr, length, "开始执行合约 收到的参数");
  const contract = contractList[id];
  log().info(contract, "runcontract 取到的合约");
  // todo: add env from ptr & length
  const result = contract.exports.main();
  return result;
};
