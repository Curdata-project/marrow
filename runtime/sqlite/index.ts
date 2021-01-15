import * as sqlite from "sqlite3";
import * as protobuf from "protobufjs";

import { wasm_exports } from "../index";
import { getValue, setValue, getValueByBytes } from "../utils";

const db = new sqlite.Database("test.db");

export const  _sql_run_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const Sql =  getSqlByProto(ptr, path_length).toJSON();
  console.log(Sql.sql, "sql");
  db.run(Sql.sql, (err) => {
    if (err) {
      console.log(err, "err");
      const { ptr, length } = setValue("fail");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    } else {
      const { ptr, length } = setValue("ok");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    }
  });
};

export const _sql_query_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const sql = getValue(ptr, path_length);
  db.all(sql, (err, data) => {
    if (err) {
      const { ptr, length } = setValue("fail");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    } else {
      const { ptr, length } = setValue(JSON.stringify(data));
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    }
  });
};

export const _sql_operate_callback = (ptr: number, size: number, fn: number, addr: number) => {
  const tableName = getValue(ptr, size);
  db.get(`SELECT name FROM sqlite_master WHERE type='table' AND name='${tableName}'`, (error, row) => {
    if (error) {
      console.log(error);
    }
    if (row === undefined) {
      wasm_exports.call_sql_operate_callback_fn(1, fn, addr);
      console.log("表不存在")
    } else {
      wasm_exports.call_sql_operate_callback_fn(0, fn, addr);
      console.log("表存在")
    }
  });
};

const getSqlByProto = (ptr: number, length: number) => {
  const buffer = getValueByBytes(ptr, length);
  const typedArray = new Uint8Array(buffer);
  try {
    const root = protobuf.loadSync("common/proto/common.proto");
    const Sql = root.lookupType("Sql");
    return Sql.decodeDelimited(typedArray);
  } catch (error) {
    console.log("protobuf decode sql error", error);
  }
};
