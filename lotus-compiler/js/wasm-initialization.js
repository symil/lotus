import { FileSystemManager } from './file-system-manager';
import { readStringFromMemory, writeNetworkEventToBuffer, writeWindowEventToBuffer } from './js-wasm-communication';
import { MemoryBuffer } from './memory-buffer';
import { NetworkManager } from './network-manager';
import { Renderer } from './renderer';
import { WindowManager } from './window-manager';

export async function initializeWasm(wasm, { log, getWindow, createWebSocket, createWebSocketServer }) {
    let instance = null;
    let getMemory = () => new Int32Array(instance.exports.memory.buffer);
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
    - `getMemory()`: returns the WASM instance memory as an Int32Array
    - `getWindow()`: returns the `window` main object of the browser
    - `createWebSocket(url)`: create a WebSocket connecting to the specified url
    - `createWebSocketServer(port)`: create a WebSocket server listening on the specified port
    - `getPathModule()`: returns the `path` module of Node.js
    - `getFileSystemModule()`: returns the `fs` module of Node.js
    - `getFileSystemRootPath()`: returns the path of the root directory where files should be stored
*/
function getWasmImportsObject(env) {
    let windowManager = null;
    let renderer = null;
    let networkManager = new NetworkManager(env);
    let fileSystemManager = new FileSystemManager(env);

    return {
        utils: {
            assert(line, value) {
                if (!value) {
                    console.error(`line ${line}: test failed`);
                }
            },
            float_to_string(value, resultAddr) {
                let memory = env.getMemory();
                let str = '' + value;

                memory[resultAddr] = str.length;

                for (let i = 0; i < str.length; ++i) {
                    memory[resultAddr + i + 2] = str.charCodeAt(i);
                }
            }
        },
        env: {
            log(stringAddr) {
                let string = readStringFromMemory(env.getMemory(), stringAddr);

                env.log(string);
            },

            log_int(value) {
                console.log(value);
            },

            get_current_time() {
                return Date.now();
            },
            
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

            poll_window_events(bufferAddr, bufferCapacity) {
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferCapacity);
                let events = windowManager.pollEvents();

                for (let event of events) {
                    writeWindowEventToBuffer(event, buffer);

                    if (buffer.getSize() > bufferCapacity) {
                        throw new Error(`event buffer overflow`);
                    }
                }

                return buffer.getSize();
            },

            draw_frame(bufferAddr, bufferSize) {
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferSize);

                renderer.drawFrameFromBuffer(buffer);
            },

            clear_renderer_cache() {
                renderer.clearCache();
            },

            create_websocket(urlAddr) {
                let url = readStringFromMemory(env.getMemory(), urlAddr);

                return networkManager.createWebSocket(url);
            },

            get_websocket_state(webSocketId) {
                return networkManager.getWebSocketState(webSocketId);
            },

            create_websocket_server(port) {
                return networkManager.createWebSocketServer(port);
            },

            send_message(webSocketId, messageAddr, messageSize) {
                let message = new Uint32Array(env.getMemory().buffer, messageAddr * 4, messageSize);

                networkManager.sendMessage(webSocketId, message);
            },

            poll_network_events(bufferAddr, bufferCapacity) {
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferCapacity);
                let events = networkManager.pollEvents();

                for (let event of events) {
                    writeNetworkEventToBuffer(event, buffer);

                    if (buffer.getSize() > bufferCapacity) {
                        throw new Error(`network buffer overflow`);
                    }
                }

                return buffer.getSize();
            },

            write_file(pathAddr, bufferAddr, bufferSize) {
                let path = readStringFromMemory(env.getMemory(), pathAddr);
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferSize);

                fileSystemManager.writeFile(path, buffer.toRegularBuffer());
            },

            read_file(pathAddr, bufferAddr, bufferCapacity) {
                let path = readStringFromMemory(env.getMemory(), pathAddr);
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferCapacity);
                let bytes = fileSystemManager.readFile(path);

                buffer.writeBuffer(bytes);

                return buffer.getSize();
            }
        }
    };
}