import { wasm_exports } from "../index";

const contractList: any[] = [];

export const getContract = async (ptr: number, length: number) => {
  const buffer = wasm_exports.memory.buffer.slice(ptr, ptr + length);
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
