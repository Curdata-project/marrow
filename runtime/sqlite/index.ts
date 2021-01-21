import * as sqlite from "sqlite3";
import * as protobuf from "protobufjs";

import { wasm_exports } from "../index";
import { getValue, setValue, getValueByBytes } from "../utils";
import { log } from "../utils/log";

const db = new sqlite.Database("test.db");

export const  _sql_run_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const Sql =  getSqlByProto(ptr, path_length).toJSON();
  log().info(Sql.sql, "sql");
  db.run(Sql.sql, (err) => {
    if (err) {
      log().error(err, "err");
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
  log().info(sql, "db.all sql");
  db.all(sql, (err, data) => {
    if (err) {
      log().info(err, "db.all sql fail");
      const { ptr, length } = setValue("fail");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    } else {
      log().info(data, "db.all sql success");
      const { ptr, length } = setValue(JSON.stringify(data));
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
      wasm_exports._wasm_free(ptr, length);
    }
  });
};

export const _sql_operate_callback = (ptr: number, size: number, fn: number, addr: number) => {
  const tableName = getValue(ptr, size);
  log().info(`judge table ${tableName} does it exist`);
  db.get(`SELECT name FROM sqlite_master WHERE type='table' AND name='${tableName}'`, (error, row) => {
    if (error) {
      log().error(error);
    }
    if (row === undefined) {
      log().info(`${tableName}表不存在`);
      wasm_exports.call_sql_operate_callback_fn(1, fn, addr);
    } else {
      log().info(`${tableName}表存在`);
      wasm_exports.call_sql_operate_callback_fn(0, fn, addr);
    }
  });
};

const getSqlByProto = (ptr: number, length: number) => {
  log().info(ptr, length, "sql proto from wasm");
  const buffer = getValueByBytes(ptr, length);
  const typedArray = new Uint8Array(buffer);
  try {
    const root = protobuf.loadSync("common/proto/common.proto");
    const Sql = root.lookupType("Sql");
    const result = Sql.decodeDelimited(typedArray);
    if (result) {
      return result;
    }
  } catch (error) {
    log().error("protobuf decode sql error", error);
  }
};
