import path from 'path';
import fs from 'fs';
import express from 'express';
import { WebSocketServer } from 'ws';
import { initializeWasm } from './wasm-initialization';
import { SERVER_CONFIG_FILE_NAME } from '../../lotus-cli/src/constants';

async function main() {
    let serverDirectory = path.dirname(process.argv[1]);
    let httpPort = parseInt(process.argv[2]);
    let clientDirectory = path.resolve(process.argv[3]);

    let wasmPath = path.join(clientDirectory, 'module.wasm');
    let configPath = path.join(serverDirectory, SERVER_CONFIG_FILE_NAME);
    let config = JSON.parse(fs.readFileSync(configPath));
    let fileSystemRootPath = path.join(process.env.HOME, '.cache', 'lotus', config.name);
    let wasmEnv = makeWasmEnv({ fileSystemRootPath });
    let wasmBytes = fs.readFileSync(wasmPath, null);
    let wasmInstance = await initializeWasm(wasmBytes, wasmEnv);
    let httpServer = express();

    wasmInstance.exports.start_server();
    setInterval(() => wasmInstance.exports.update_server(), 10);
    
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