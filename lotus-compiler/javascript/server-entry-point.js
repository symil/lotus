import path from 'path';
import fs from 'fs';
import https from 'https';
import url from 'url';
import { WebSocketServer } from 'ws';
import { openServer } from 'outpost';
import { initializeWasm } from './wasm-initialization';
import { SERVER_REFRESH_RATE } from './constants';

const { HOME, OUTPOST_APP_NAME, OUTPOST_CLIENT_DIR } = process.env;

async function main() {
    let webSocketServerList = [];
    let fileSystemRootPath = path.join(HOME, '.lotus-server-data', OUTPOST_APP_NAME);
    let wasmPath = path.join(OUTPOST_CLIENT_DIR, 'module.wasm');
    let wasmEnv = makeWasmEnv({ fileSystemRootPath }, webSocketServerList);
    let wasmBytes = fs.readFileSync(wasmPath, null);
    let wasmInstance = await initializeWasm(wasmBytes, wasmEnv);

    wasmInstance.exports.start_server();

    let update = () => {
        wasmInstance.exports.update_server();
        setTimeout(update, SERVER_REFRESH_RATE);
    };

    update();

    await openServer((_app, server) => {
        server.on('upgrade', (request, socket, head) => {
            let { pathname } = url.parse(request.url);

            if (pathname === '/ws') {
                for (let wsServer of webSocketServerList) {
                    wsServer.handleUpgrade(request, socket, head, (ws) => {
                        wsServer.emit('connection', ws, request);
                    });
                }
            } else {
                socket.destroy();
            }
        });
    });
}

function makeWasmEnv({ fileSystemRootPath }, webSocketServerList) {
    return {
        log(string) {
            console.log(string);
        },
        createWebSocketServer() {
            let server = new WebSocketServer({ noServer: true });

            webSocketServerList.push(server);

            return server;
        },
        getProcess() {
            return process;
        },
        getHttpsModule() {
            return https;
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