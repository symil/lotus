function main() {
    console.log('HELLO');
    // let env = { log, createWebSocket };
    // console.log(import('./module.wasm'));
    // let instance = initializeWasm(import('./module.wasm'), env);
}

function createWebSocket(url) {
    return new WebSocket(url);
}

function log(string) {
    return console.log(string);
}

main();