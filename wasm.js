import fs from 'fs';
import { WASI } from 'wasi';

async function main() {
    let wasi = new WASI();
    let importObject = {
        log: {
            i32: value => console.log(value)
        }
    };

    let wasm = await WebAssembly.compile(fs.readFileSync('./build/module.wasm'));
    let instance = await WebAssembly.instantiate(wasm, importObject);

    wasi.start(instance);
}

main();