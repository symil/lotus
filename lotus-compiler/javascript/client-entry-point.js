import { initializeWasm } from './wasm-initialization';

async function main() {
    let env = makeWasmEnv();
    let instance = await initializeWasm(fetch('./module.wasm'), env);
    let update = () => {
        let now = performance.now();
        instance.exports.update_client();
        let elapsed = performance.now() - now;

        window.requestAnimationFrame(update);
    };

    instance.exports.start_client();
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
    }
}

main();