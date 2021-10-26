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
            char(value) {
                env.log(String.fromCodePoint(value));
            },
            string(addr) {
                let memory = env.getMemory();
                let length = memory[addr];
                let codes = new Array(length);

                for (let i = 0; i < length; ++i) {
                    codes[i] = memory[addr + 2 + i];
                }

                let string = String.fromCodePoint(...codes);

                env.log(string);
            }
        },
        utils: {
            float_to_string(value, result_addr) {
                let memory = env.getMemory();
                let str = '' + value;

                memory[result_addr] = str.length;

                for (let i = 0; i < str.length; ++i) {
                    memory[result_addr + i + 2] = str.charCodeAt(i);
                }
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
    let start = performance.now();

    env.init(instance);
    wasi.start(instance);

    let time = performance.now() - start;

    return { instance, time };
}