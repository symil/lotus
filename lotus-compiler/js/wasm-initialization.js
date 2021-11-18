export async function initializeWasm(wasmPath, { log, getWindow, createWebSocket, createWebSocketServer }) {
    let instance = null;
    let getMemory = () => new Uint32Array(instance.exports.memory.buffer);
    let env = { log, getMemory, getWindow, createWebSocket, createWebSocketServer };
    let imports = getWasmImportsObject(env);
    let wasm = await WebAssembly.compile(fs.readFileSync(wasmPath));

    instance = await WebAssembly.instantiate(wasm, imports);
    instance.exports.initialize();

    return instance;
}

/*
    `env` must have the following methods:
    - `log(string)`: log the string in the console
    - `getMemory()`: returns the WASM instance memory as an Uint32Array
    - `getWindow()`
    - `createWebSocket()`
    - `createWebSocketServer()`
*/
function getWasmImportsObject(env) {
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
            },

            get_current_time() {
                return Date.now();
            }
        },
        utils: {
            float_to_string(value, resultAddr) {
                let memory = env.getMemory();
                let str = '' + value;

                memory[resultAddr] = str.length;

                for (let i = 0; i < str.length; ++i) {
                    memory[resultAddr + i + 2] = str.charCodeAt(i);
                }
            }
        }
    };
}