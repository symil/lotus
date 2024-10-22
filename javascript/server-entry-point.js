import path from 'path';
import fs, { existsSync, readFileSync } from 'fs';
import http from 'http';
import https from 'https';
import url from 'url';
import express from 'express';
import { WebSocketServer } from 'ws';
import { initializeWasm } from './wasm-initialization.js';
import { DEFAULT_ASSET_PROXY_BASE_PATH, DEFAULT_HTTP_PORT, DEFAULT_WEBSOCKET_UPGRADE_PATH, OUTPUT_WASM_FILE_NAME, SERVER_CLIENT_FILES_PATH, SERVER_REFRESH_RATE } from './constants.js';
import { getWasmExports } from './utils.js';

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

async function main() {
    process.chdir(__dirname);

    let webSocketServerList = [];
    let fileSystemRootPath = '.';
    let wasmPath = path.join('.', OUTPUT_WASM_FILE_NAME);
    let wasmEnv = makeWasmEnv({ fileSystemRootPath }, webSocketServerList);
    let wasmBytes = fs.readFileSync(wasmPath, null);
    let wasmInstance = await initializeWasm(wasmBytes, wasmEnv);
    let exports = getWasmExports(wasmInstance);

    exports.start_server();

    let update = () => {
        exports.update_server();
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
    };
}

async function openServer(options) {
    if (typeof options === 'function') {
        options = { callback: options };
    }

    let parameters = Object.assign({
        callback: () => {},
        logging: false,
        port: DEFAULT_HTTP_PORT,
        rootDirectory: SERVER_CLIENT_FILES_PATH,
        sslCertificatePath: null,
        sslPrivateKeyPath: null,
        assetProxy: false,
        webSockets: false,
        useConnectionFramework: false,
        webSocketApi: {},
        createUser: () => ({})
    }, options);

    if (typeof parameters.assetProxy === 'boolean') {
        parameters.assetProxy = { enabled: parameters.assetProxy };
    }

    if (typeof parameters.webSockets === 'boolean') {
        parameters.webSockets = { enabled: parameters.webSockets };
    }

    let assetProxyParameters = Object.assign({
        enabled: false,
        basePath: DEFAULT_ASSET_PROXY_BASE_PATH
    }, parameters.assetProxy);

    let webSocketsParameters = Object.assign({
        enabled: false,
        upgradePath: DEFAULT_WEBSOCKET_UPGRADE_PATH
    }, parameters.webSockets);

    let server = {};

    // @ts-ignore
    server.expressApp = express();
    server.httpServer = await createHttpServer(server.expressApp, parameters.sslCertificatePath, parameters.sslPrivateKeyPath);

    server.expressApp.use(express.text());
    server.expressApp.use(express.json());
    configureAssetProxy(server.expressApp, assetProxyParameters);
    await parameters.callback(server.expressApp, server.httpServer);

    if (parameters.rootDirectory) {
        server.expressApp.use(express.static(parameters.rootDirectory));
    }

    if (webSocketsParameters.enabled) {
        server.webSocketServer = createWebSocketServer(server.httpServer, webSocketsParameters.upgradePath);
    }

    return new Promise(resolve => {
        server.httpServer.listen(parameters.port, () => {
            if (parameters.logging) {
                console.log(`=> listening on port ${parameters.port}...`);
            }
            resolve(server);
        });
    });
}

function configureAssetProxy(app, parameters) {
    if (parameters.enabled) {
        let prefix = `${parameters.basePath}`;

        if (!prefix.startsWith('/')) {
            prefix = '/' + prefix;
        }

        if (!prefix.endsWith('/')) {
            prefix += '/';
        }

        app.get(`${prefix}*`, async (req, res) => {
            let targetUrlIndexStart = req.originalUrl.indexOf(prefix) + prefix.length;

            if (targetUrlIndexStart < prefix.length) {
                res.sendStatus(400);
                return;
            }

            let targetUrlEncoded = req.originalUrl.substring(targetUrlIndexStart);
            let targetUrl = decodeURI(targetUrlEncoded);

            try {
                let assetResponse = await fetch(targetUrl);

                assetResponse.headers.forEach((value, name) => res.setHeader(name, value));
                // res.setHeader('Access-Control-Allow-Origin', '*');
                // res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
                // res.setHeader('Access-Control-Allow-Headers', 'Origin,DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range');
                // res.setHeader('Access-Control-Expose-Headers', 'Content-Length,Content-Range');

                if (assetResponse.body) {
                    let bytes = await assetResponse.arrayBuffer();
                    let buffer = new Uint8Array(bytes);

                    res.write(buffer);
                }

                res.status(assetResponse.status);
                res.end();
            } catch (e) {
                res.sendStatus(404);
            }
        });
    }
}

async function createHttpServer(app, certPath, keyPath) {
    let httpServer = null;

    if (certPath && keyPath && existsSync(certPath) && existsSync(keyPath)) {
        try {
            let cert = readFileSync(certPath, 'utf8');
            let key = readFileSync(keyPath, 'utf8');

            httpServer = https.createServer({ cert, key }, app);
        } catch (e) {
            console.log(e);
            console.log('error: unable to open ssl key & certificate, defaulting to http');
        }
    }

    if (!httpServer) {
        httpServer = http.createServer(app);
    }

    return httpServer;
}

function createWebSocketServer(httpServer, upgradePath) {
    // @ts-ignore
    let webSocketServer = new WebSocketServer({ noServer: true });

    httpServer.on('upgrade', (request, socket, head) => {
        if (request.url === upgradePath) {
            webSocketServer.handleUpgrade(request, socket, head, (ws) => {
                webSocketServer.emit('connection', ws, request);
            });
        } else {
            socket.destroy();
        }
    });

    return webSocketServer;
}

main();