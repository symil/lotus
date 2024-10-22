import { OUTPUT_WASM_FILE_NAME } from './constants.js';
import { initializeWasm } from './wasm-initialization.js';

async function main() {
    let env = makeWasmEnv();
    let instance = await initializeWasm(fetch(`./${OUTPUT_WASM_FILE_NAME}`), env);
    /** @type {any} */
    let exports = instance.exports;
    let update = () => {
        let now = performance.now();
        exports.update_client();
        let elapsed = performance.now() - now;

        window.requestAnimationFrame(update);
    };

    exports.start_client();
    update();
}

function makeWasmEnv() {
    return {
        getWindow() {
            return window;
        },
        createWebSocket(url, options) {
            return new WebSocket(url, options);
        },
        log(string) {
            return console.log(string);
        },
    };
}

main();