const fs = require('fs');

async function main () {
    let actor_bin_code = fs.readFileSync('target/wasm32-unknown-unknown/release/demo.wasm');
    
    let actor_bin_instance = null;

    let import_object = {
        wstd: {
            print(ptr, length) {
                const value = actor_bin_instance.exports.memory.buffer.slice(ptr, ptr + length);
                let utf8decoder = new TextDecoder();
                console.log(utf8decoder.decode(value));
                // console.log(ptr, length);
            },
            _read_file_callback(fn, addr, ptr, path_length) {
                const value = actor_bin_instance.exports.memory.buffer.slice(ptr, ptr + path_length);
                let utf8decoder = new TextDecoder();
                path = utf8decoder.decode(value);

                fs.readFile(path, (err, data) => {
                    console.log(fn, addr)
                    actor_bin_instance.exports.call_read_file_callback_fn(fn, addr, 1)
                });
                return 0;
            }
        }
    };

    actor_bin_instance = (await WebAssembly.instantiate(actor_bin_code, import_object)).instance;

    actor_bin_instance.exports._entry()
}

main()
