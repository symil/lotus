import { initializeWasm } from './wasm-initialization';

async function main() {
    let env = makeWasmEnv();
    let instance = await initializeWasm(fetch('./module.wasm'), env);
    let update = () => {
        instance.exports.update_client();
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
        createWebSocket(url) {
            return new WebSocket(url);
        },
        log(string) {
            return console.log(string);
        },
    }
}

main();