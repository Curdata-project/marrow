import * as sqlite from "sqlite3";

import { wasm_exports } from "../index";
import { getValue, setValue } from "../utils";

const db = new sqlite.Database("test.db");

export const _sql_run_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const sql = getValue(ptr, path_length);
  db.run(sql, (err) => {
    if (err) {
      const { ptr, length } = setValue("fail");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
    } else {
      const { ptr, length } = setValue("ok");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
    }
  });
};

export const _sql_query_callback = (ptr: number, path_length: number, fn: number, addr: number) => {
  const sql = getValue(ptr, path_length);
  db.all(sql, (err, data) => {
    if (err) {
      const { ptr, length } = setValue("fail");
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
    } else {
      const { ptr, length } = setValue(JSON.stringify(data));
      wasm_exports.call_sql_callback_fn(ptr, length, fn, addr);
    }
  });
};
