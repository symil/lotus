import { readStringFromBuffer, writeWindowEventToBuffer } from './js-wasm-communication';
import { MemoryBuffer } from './memory-buffer';
import { Renderer } from './renderer';
import { WindowManager } from './window-manager';

export async function initializeWasm(wasm, { log, getWindow, createWebSocket, createWebSocketServer }) {
    let instance = null;
    let getMemory = () => instance.exports.memory.buffer;
    let env = { getMemory, log, getWindow, createWebSocket, createWebSocketServer };
    let imports = getWasmImportsObject(env);

    if (WebAssembly.instantiateStreaming) {
        instance = (await WebAssembly.instantiateStreaming(wasm, imports)).instance;
    } else {
        instance = (await WebAssembly.instantiate(wasm, imports)).instance;
    }

    instance.exports.initialize();

    return instance;
}

/*
    `env` must have the following methods:
    - `log(string)`: log the string in the console
    - `getMemory()`: returns the WASM instance memory as an ArrayBuffer
    - `getWindow()`
    - `createWebSocket()`
    - `createWebSocketServer()`
*/
function getWasmImportsObject(env) {
    let windowManager = null;
    let renderer = null;
    let webSocket = null;
    let webSocketServer = null;

    return {
        env: {
            log(stringAddr) {
                let buffer = new MemoryBuffer(env.getMemory(), stringAddr);
                let string = readStringFromBuffer(buffer);

                env.log(string);
            },

            log_int(value) {
                console.log(value);
            },

            get_current_time() {
                return Date.now();
            }
        },
        utils: {
            assert(line, value) {
                if (!value) {
                    console.error(`line ${line}: test failed`);
                }
            },
            float_to_string(value, resultAddr) {
                let memory = new Uint32Array(env.getMemory());
                let str = '' + value;

                memory[resultAddr] = str.length;

                for (let i = 0; i < str.length; ++i) {
                    memory[resultAddr + i + 2] = str.charCodeAt(i);
                }
            }
        },
        client: {
            init_window(aspectRatio) {
                windowManager = new WindowManager(env.getWindow(), aspectRatio);
                renderer = new Renderer(windowManager);

                windowManager.start();
            },

            get_window_width() {
                return windowManager.getWidth();
            },

            get_window_height() {
                return windowManager.getHeight();
            },

            poll_events(bufferLength, bufferAddr) {
                let events = windowManager.pollEvents();
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr);

                for (let event of events) {
                    writeWindowEventToBuffer(event, buffer);

                    if (buffer.getLength() > bufferLength) {
                        throw new Error(`event buffer overflow`);
                    }
                }

                return buffer.getLength();
            }
        }
    };
}