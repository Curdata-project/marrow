const sqlite = require("sqlite3");

const { actor_bin_instance } = require("../index");
const { getValue, setValue } = require("./utils");

const { call_sqlite_callback_fn } = actor_bin_instance.exports;

const db = new sqlite.Database("test.db");

const _sqlite_callback = (fn, addr, ptr, path_length) => {
  const sql = getValue(ptr, path_length);
  db.all(sql, (err, data) => {
    if (err) {
      console.log(err);
    }
    const { ptr, length } = setValue(data);
    call_sqlite_callback_fn(fn, addr, ptr, length);
  })
};

module.exports = {
  _sqlite_callback,
};
