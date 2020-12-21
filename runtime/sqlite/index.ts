import * as sqlite from "sqlite3";

import { wasm_exports } from "../index";
import { getValue, setValue } from "../utils";

const db = new sqlite.Database("test.db");

export const _sqlite_callback = (fn: any, addr: any, ptr: number, path_length: number) => {
  const sql = getValue(ptr, path_length);
  db.all(sql, (err, data) => {
    if (err) {
      console.log(err);
    }
    // const { ptr, length } = setValue(data);
    // call_sqlite_callback_fn(fn, addr, ptr, length);
  })
};
