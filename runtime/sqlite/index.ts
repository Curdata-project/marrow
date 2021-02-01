import * as sqlite from "sqlite3";
import * as protobuf from "protobufjs";

import { getWasmExport } from "../storage";
import { getValue, setValue, getValueByBytes } from "../utils";
import { log } from "../utils/log";

const db = new sqlite.Database("test.db");

export const  _sql_run_callback = (moduleName: string) => {
  return function _sql_run_callback (ptr: number, path_length: number, fn: number, addr: number) {
    const wasm_exports = getWasmExport(moduleName);
    const Sql =  getSqlByProto(moduleName, ptr, path_length).toJSON();
    log().info(Sql, "sql");
    let params = [];

    if (Sql.params) {
      params = Sql.params.map((item: any) => {
        if (item.tp === "bytes") {
          return Buffer.from(item.buffer, "base64");
        }
        if (item.tp === "number") {
          return Number(item.number);
        }
        if (item.tp === "string") {
          return item.s;
        }
      });
    }

    db.run(Sql.sql, params, (err: any) => {
      if (err) {
        log().error(err, "err");
        const { ptr, length } = setValue(moduleName, "fail");
        wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
        wasm_exports._wasm_free(ptr, length);
      } else {
        const { ptr, length } = setValue(moduleName, "ok");
        wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
        wasm_exports._wasm_free(ptr, length);
      }
    });
  };
};

export const _sql_query_callback = (moduleName: string) => {
  return function _sql_query_callback (ptr: number, path_length: number, fn: number, addr: number) {
    const wasm_exports = getWasmExport(moduleName);
    const sql = getValue(moduleName, ptr, path_length);
    log().info(sql, "db.all sql");
    db.all(sql, (err, data) => {
      if (err) {
        log().info(err, "db.all sql fail");
        const { ptr, length } = setValue(moduleName, "fail");
        wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
        wasm_exports._wasm_free(ptr, length);
      } else {
        log().info(data, "db.all sql success");
        const { ptr, length } = setValue(moduleName, JSON.stringify(data));
        wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
        wasm_exports._wasm_free(ptr, length);
      }
    });
  };
};

export const _sql_operate_callback = (moduleName: string) => {
  return function _sql_operate_callback (ptr: number, size: number, fn: number, addr: number) {
    const wasm_exports = getWasmExport(moduleName);
    const tableName = getValue(moduleName, ptr, size);
    log().info(`judge table ${tableName} does it exist`);
    db.get(`SELECT name FROM sqlite_master WHERE type='table' AND name='${tableName}'`, (error, row) => {
      if (error) {
        log().error(error);
      }
      if (row === undefined) {
        log().info(`${tableName} table does not exist`);
        wasm_exports.call_sql_operate_callback_fn(1, fn, addr);
      } else {
        log().info(`${tableName} table exist`);
        wasm_exports.call_sql_operate_callback_fn(0, fn, addr);
      }
    });
  };
};

const getSqlByProto = (moduleName: string, ptr: number, length: number) => {
  log().info(ptr, length, "sql proto from wasm");
  const buffer = getValueByBytes(moduleName, ptr, length);
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
