const axios = request("axios");

const { actor_bin_instance } = require("../../index");
const { getValue, setValue } = require("../utils");

const { call_request_callback_fn } = actor_bin_instance.exports;

const _request_callback = (fn, addr, ptr, path_length) => {
  const value = getValue(ptr, path_length);
  const arg = JSON.parse(value);
  axios({...arg}).then(result => {
    const { ptr, length } = setValue(result);
    call_request_callback_fn(fn, addr, ptr, length);
  }).catch(error => {
    const { ptr, length } = setValue(error);
    call_request_callback_fn(fn, addr, ptr, length);
  })
};

module.exports = {
  _request_callback,
};
