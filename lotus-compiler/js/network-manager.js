export class NetworkManager {
    constructor({ createWebSocket, createWebSocketServer }) {
        this._createWebSocket = createWebSocket;
        this._createWebSocketServer = createWebSocketServer;
        this._webSocketIdCounter = 1;
        this._webSockets = new Map();
        this._webSocketServerIdCounter = 1;
        this._webSocketServers = new Map();
        this._events = [];

        this._stateUpdateMessageIndex = -1;
    }

    createWebSocket(url) {
        let webSocket = this._createWebSocket(url);

        webSocket.binaryType = 'arraybuffer';

        return this._initWebSocket(webSocket);
    }

    getWebSocketState(webSocketId){ 
        return this._webSockets.get(webSocketId)?.readyState;
    }

    createWebSocketServer(port) {
        let webSocketServer = this._createWebSocketServer({ port });
        let id = this._webSocketServerIdCounter++;

        webSocketServer.on('connection', (webSocket) => {
            let id = this._initWebSocket(webSocket);

            this._registerEvent(id, 'open', null);
        });

        this._webSocketServers.set(id, webSocketServer);

        return id;
    }

    sendMessage(webSocketId, message) {
        let webSocket = this._webSockets.get(webSocketId);
        
        if (webSocket) {
            if (webSocket.readyState === 0) {
                webSocket.addEventListener('open', () => {
                    webSocket.send(message);
                });
            } else {
                webSocket.send(message);
            }
        } else {
            throw new Error(`cannot send message: websocket is closed`);
        }
    }

    pollEvents() {
        let result = this._events;

        this._events = [];
        this._stateUpdateMessageIndex = -1;

        return result;
    }

    close() {

    }

    _registerEvent(webSocketId, messageType, messagePayload) {
        this._events.push({
            webSocketId,
            messageType,
            messagePayload
        });

        if (messageType === 'message') {
            if (this._stateUpdateMessageIndex !== -1) {
                this._events.splice(this._stateUpdateMessageIndex, 1);
            }

            this._stateUpdateMessageIndex = this._events.length - 1;
        }
    }

    _initWebSocket(webSocket) {
        let id = this._webSocketIdCounter++;

        this._webSockets.set(id, webSocket);

        webSocket.addEventListener('open', () => {
            this._registerEvent(id, 'open', null);
        });
        webSocket.addEventListener('close', () => {
            this._registerEvent(id, 'close', null);
            this._webSockets.delete(id);
        });
        webSocket.addEventListener('message', (event) => {
            let buffer = new Uint8Array(event.data);

            this._registerEvent(id, 'message', buffer);
        });

        return id;
    }
}