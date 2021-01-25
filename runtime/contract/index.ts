import { getWasmExport } from "../storage";

const contractList: any[] = [];

export const getContract = async (ptr: number, length: number) => {
  const wasm_exports = getWasmExport();
  const buffer = wasm_exports.memory.buffer.slice(ptr, ptr + length);
  console.log(buffer, ptr, length, "from get contract");
  const { instance } = await WebAssembly.instantiate(buffer, {});
  contractList.push(instance);
  return contractList.length - 1;
};

export const runContract = (id: number, ptr: number, length: number) => {
  const contract = contractList[id];
  console.log(contract, "contract");
  // todo: add env from ptr & length
  const result = contract.exports.main();
  return result;
};
