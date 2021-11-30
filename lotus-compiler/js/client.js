import { initializeWasm } from './wasm-initialization';

async function main() {
    let env = { log, getWindow, createWebSocket };
    let instance = await initializeWasm(fetch('./module.wasm'), env);
    let update = () => {
        instance.exports.update_client();
        window.requestAnimationFrame(update);
    };

    instance.exports.start_client();
    update();
}

function getWindow() {
    return window;
}

function createWebSocket(url) {
    return new WebSocket(url);
}

function log(string) {
    return console.log(string);
}

main();