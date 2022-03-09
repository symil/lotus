import { FileSystemManager } from './file-system-manager';
import { KEYBOARD_CODES, readStringFromMemory, writeNetworkEventToBuffer, writeWindowEventToBuffer } from './js-wasm-communication';
import { KeyboardManager } from './keyboard-manager';
import { MemoryBuffer } from './memory-buffer';
import { MemoryManager } from './memory-manager';
import { NetworkManager } from './network-manager';
import { Renderer } from './renderer';
import { decodeStringToUint32Array, encodeUint32ArrayToString } from './utils';
import { WindowManager } from './window-manager';

export async function initializeWasm(wasm, userEnv) {
    let instance = null;
    let getMemory = () => new Int32Array(instance.exports.memory.buffer);
    let env = { ...userEnv, getMemory };
    let imports = await getWasmImportsObject(env);

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
    - `getMemory()`: return the WASM instance memory as an Int32Array
    - `getWindow()`: return the `window` main object of the browser
    - `createWebSocket(url)`: create a WebSocket connecting to the specified url
    - `createWebSocketServer(port)`: create a WebSocket server listening on the specified port
    - `getProcess()` return the `process` module of Node.js
    - `getPathModule()`: return the `path` module of Node.js
    - `getFileSystemModule()`: return the `fs` module of Node.js
    - `getFileSystemRootPath()`: return the path of the root directory where files should be stored
*/
async function getWasmImportsObject(env) {
    let memoryManager = new MemoryManager(env);
    let windowManager = new WindowManager(env);
    let renderer = new Renderer(windowManager);
    let networkManager = new NetworkManager(env);
    let fileSystemManager = new FileSystemManager(env);
    let keyboardManager = await KeyboardManager.new(env);

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
                windowManager.init(aspectRatio);
            },

            get_window_width() {
                return windowManager.getWidth();
            },

            get_window_height() {
                return windowManager.getHeight();
            },

            set_window_title(stringAddr) {
                let title = readStringFromMemory(env.getMemory(), stringAddr);

                windowManager.setTitle(title);
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
                let message = new Int32Array(env.getMemory().buffer, messageAddr * 4, messageSize);

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
            },

            set_local_storage_item(keyAddr, bufferAddr, bufferSize) {
                let key = readStringFromMemory(env.getMemory(), keyAddr);
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferSize);
                let encoded = encodeUint32ArrayToString(buffer.toUint32Array());
                let localStorage = env.getWindow().localStorage;

                localStorage.setItem(key, encoded);
            },

            remove_local_storage_item(keyAddr) {
                let key = readStringFromMemory(env.getMemory(), keyAddr);
                let localStorage = env.getWindow().localStorage;

                localStorage.removeItem(key);
            },

            get_local_storage_item(keyAddr, bufferAddr, bufferCapacity) {
                let key = readStringFromMemory(env.getMemory(), keyAddr);
                let buffer = new MemoryBuffer(env.getMemory(), bufferAddr, bufferCapacity);
                let localStorage = env.getWindow().localStorage;
                let item = localStorage.getItem(key) || '';
                let decoded = decodeStringToUint32Array(item);
                let decodedAsIntArray = new Int32Array(decoded.buffer);

                for (let n of decodedAsIntArray) {
                    buffer.write(n);
                }

                return buffer.getSize();
            },

            clear_local_storage() {
                let localStorage = env.getWindow().localStorage;

                localStorage.clear();
            },

            process_exit(code) {
                if (env.getProcess) {
                    env.getProcess().exit(code);
                }
            },

            prompt(messageAddr, bufferAddr) {
                let window = env.getWindow();
                let message = memoryManager.readString(messageAddr);
                let buffer = memoryManager.readBuffer(bufferAddr);
                let result = window.prompt(message) || '';

                buffer.writeString(result);
            },
            
            get_href(bufferAddr) {
                let window = env.getWindow();
                let buffer = memoryManager.readBuffer(bufferAddr);

                buffer.writeString(window.location.href);
            },

            get_key_value(keyValue, bufferAddr) {
                let key = KEYBOARD_CODES[keyValue];
                let buffer = memoryManager.readBuffer(bufferAddr);
                let name = keyboardManager.getKeyValue(key).toUpperCase();

                buffer.writeString(name);
            }
        }
    };
}