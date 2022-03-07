import path from 'path';
import fs from 'fs';
import express from 'express';
import { WebSocketServer } from 'ws';
import { initializeWasm } from './wasm-initialization';
import { SERVER_ENTRY_PATH } from './constants';

const { HOME, OUTPOST_PORT, OUTPOST_CLIENT_DIR, OUTPOST_PROJECT_ID } = process.env;

async function main() {
    let httpPort = parseInt(OUTPOST_PORT);
    let clientDirectory = OUTPOST_CLIENT_DIR;
    let fileSystemRootPath = path.join(HOME, '.lotus-server-data', OUTPOST_PROJECT_ID);

    let wasmPath = path.join(clientDirectory, 'module.wasm');
    let wasmEnv = makeWasmEnv({ fileSystemRootPath });
    let wasmBytes = fs.readFileSync(wasmPath, null);
    let wasmInstance = await initializeWasm(wasmBytes, wasmEnv);
    let httpServer = express();

    wasmInstance.exports.start_server();
    setInterval(() => wasmInstance.exports.update_server(), SERVER_ENTRY_PATH);
    
    httpServer.use(express.static(clientDirectory));
    httpServer.listen(httpPort, () => {
        console.log(`=> http server listening on port ${httpPort}...`);
    });
}

function makeWasmEnv({ fileSystemRootPath }) {
    return {
        log(string) {
            console.log(string);
        },
        createWebSocketServer(options) {
            return new WebSocketServer(options);
        },
        getProcess() {
            return process;
        },
        getPathModule() {
            return path;
        },
        getFileSystemModule() {
            return fs;
        },
        getFileSystemRootPath() {
            return fileSystemRootPath;
        }
    }
}

main();