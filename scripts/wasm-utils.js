import fs from 'fs';
import { WASI } from 'wasi';
import { WasmEnv } from './wasm-env';

function getImportsObject(env) {
    return {
        log: {
            empty() {
                env.log("");
            },
            bool(value) {
                if (value === 0) {
                    env.log("false");
                } else {
                    env.log("true");
                }
            },
            int(value) {
                env.log(value);
            },
            float(value) {
                env.log(value);
            },
            string(addr) {
                let memory = env.getMemory();
                let length = memory[addr];
                let codes = new Array(length);

                for (let i = 0; i < length; ++i) {
                    codes[i] = memory[addr + 1 + i];
                }

                let string = String.fromCodePoint(...codes);

                env.log(string);
            }
        }
    };
}

export async function runWasmFile(wasmPath, { log = console.log } = {}) {
    let env = new WasmEnv({ log });
    let imports = getImportsObject(env);
    let wasi = new WASI();
    let wasm = await WebAssembly.compile(fs.readFileSync(wasmPath));
    let instance = await WebAssembly.instantiate(wasm, imports);

    env.init(instance);
    wasi.start(instance);

    return instance;
}