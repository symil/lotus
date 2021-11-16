import fs from 'fs';
import { WasmEnv } from './wasm-env';

function getImportsObject(env) {
    return {
        env: {
            log(stringAddr) {
                let memory = env.getMemory();
                let length = memory[stringAddr];
                let codes = new Array(length);

                for (let i = 0; i < length; ++i) {
                    codes[i] = memory[stringAddr + 2 + i];
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
    let wasm = await WebAssembly.compile(fs.readFileSync(wasmPath));
    let instance = await WebAssembly.instantiate(wasm, imports);
    let start = performance.now();

    env.init(instance);
    instance.exports.initialize();
    instance.exports.main();

    let time = performance.now() - start;

    return { instance, time };
}