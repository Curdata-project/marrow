import { socket } from "../rpc/server";
import { moduleCache } from "../rpc/hander";
import { getValue } from "../utils";

export const _callback_ptr_size = (index: number, ptr: number, size: number) => {
  const result = getValue(ptr, size);
  
};

export const _callback_number = (index: number, number: number) => {

};