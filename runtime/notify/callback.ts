import { log } from "../utils/log";

const __callback_u32 = (index: number, result: number) => {

};

const __callback_i32 = (index: number, result: number) => {
  log().info(index, result, "from notify i32");
};

const __callback_u64 = (index: number, result: number) => {

};

const __callback_i64 = (index: number, result: number) => {

};

const __callback_isize = (index: number, result: number) => {

};

const __callback_usize = (index: number, result: number) => {

};

const __callback_bytes = (index: number, ptr: number, length: number) => {

};

const __callback_null = (index: number) => {

};

export const mw_rt = {
  __callback_u32,
  __callback_i32,
  __callback_i64,
  __callback_u64,
  __callback_isize,
  __callback_usize,
  __callback_bytes,
  __callback_null,
};
