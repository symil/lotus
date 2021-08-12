import fs from 'fs';
import { WASI } from 'wasi';

export function getImportsObject({ log = console.log } = {}) {
    return {
        log: {
            i32(value) {
                log(value);
            }
        }
    };
}

export async function runWasmFile(wasmPath, importsObject) {
    let wasi = new WASI();
    let wasm = await WebAssembly.compile(fs.readFileSync(wasmPath));
    let instance = await WebAssembly.instantiate(wasm, importsObject);

    wasi.start(instance);

    return instance;
}