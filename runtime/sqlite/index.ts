import * as sqlite from "sqlite3";

import { wasm_exports } from "../index";
import { getValue, setValue } from "../utils";

const db = new sqlite.Database("test.db");

export const _sqlite_run_callback = (ptr: number, path_length: number, fn: any, addr: any) => {
  const sql = getValue(ptr, path_length);
  db.all(sql, (err, data) => {
    if (err) {
      console.log(err);
    }
    // const { ptr, length } = setValue(data);
    // call_sqlite_callback_fn(fn, addr, ptr, length);
  })
};
