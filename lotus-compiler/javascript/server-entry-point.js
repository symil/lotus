import path from 'path';
import fs from 'fs';
import http from 'http';
import https from 'https';
import url from 'url';
import express from 'express';
import { WebSocketServer } from 'ws';
import { initializeWasm } from './wasm-initialization';
import { SERVER_REFRESH_RATE } from './constants';

const { HOME, OUTPOST_PORT, OUTPOST_CLIENT_DIR, OUTPOST_NAME, OUTPOST_REMOTE_HOSTNAME } = process.env;

async function main() {
    let webSocketServerList = [];

    let port = parseInt(OUTPOST_PORT);
    let clientDirectory = OUTPOST_CLIENT_DIR;
    let hostname = OUTPOST_REMOTE_HOSTNAME;
    let fileSystemRootPath = path.join(HOME, '.lotus-server-data', OUTPOST_NAME);
    let wasmPath = path.join(clientDirectory, 'module.wasm');
    let wasmEnv = makeWasmEnv({ fileSystemRootPath }, webSocketServerList);
    let wasmBytes = fs.readFileSync(wasmPath, null);
    let wasmInstance = await initializeWasm(wasmBytes, wasmEnv);
    let app = express();

    app.use(express.static(clientDirectory));

    let server = createServer(app, hostname);
    
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

    wasmInstance.exports.start_server();

    let update = () => {
        wasmInstance.exports.update_server();
        setTimeout(update, SERVER_REFRESH_RATE);
    };

    update();
    
    server.listen(port, () => {
        console.log(`=> server listening on port ${port}...`);
    });
}

function createServer(app, hostname) {
    let root = '/etc/letsencrypt/live';
    let certPath = path.join(root, hostname, 'cert.pem');
    let keyPath = path.join(root, hostname, 'privkey.pem');
    let server = null;

    try {
        let cert = fs.readFileSync(certPath, 'utf8');
        let key = fs.readFileSync(keyPath, 'utf8');

        server = https.createServer({ cert, key }, app);
    } catch (e) {
        server = http.createServer(app);
    }

    return server;
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