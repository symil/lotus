import fs from 'fs';
import { WASI } from 'wasi';

export function getImportsObject({ log = console.log } = {}) {
    return {
        log: {
            bool(value) {
                if (value === 0) {
                    log("false");
                } else {
                    log("true");
                }
            },
            int(value) {
                log(value);
            },
            float(value) {
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