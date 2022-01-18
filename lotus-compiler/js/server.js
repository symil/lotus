import path from 'path';
import fs from 'fs';
import ws from 'ws';
import { initializeWasm } from './wasm-initialization';
import { FILES_DIR_NAME } from './constants';

const ROOT_DIR = path.dirname(process.argv[1]);
const FILES_DIR_PATH = path.join(ROOT_DIR, FILES_DIR_NAME);

async function main() {
    let wasmPath = path.join(ROOT_DIR, 'module.wasm');
    let env = { log, createWebSocketServer, getPathModule, getFileSystemModule, getFileSystemRootPath };
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

function getPathModule() {
    return path;
}

function getFileSystemModule() {
    return fs;
}

function getFileSystemRootPath() {
    return FILES_DIR_PATH;
}

main();