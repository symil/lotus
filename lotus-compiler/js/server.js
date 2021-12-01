import path from 'path';
import fs from 'fs';
import ws from 'ws';
import { initializeWasm } from './wasm-initialization';

async function main() {
    let wasmPath = path.join(path.dirname(process.argv[1]), 'module.wasm');
    let env = { log, createWebSocketServer };
    let instance = await initializeWasm(fs.readFileSync(wasmPath, null), env);

    instance.exports.start_server();
    setInterval(() => instance.exports.update_server(), 10);
}

function log(string) {
    console.log(string);
}

function createWebSocketServer(options) {
    return new ws.WebSocketServer(options);
}

main();