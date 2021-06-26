export class Client {
    constructor() {
        this._webSocket = null;
        this._pendingMessages = [];
    }

    async start() {
        this._webSocket = new WebSocket('ws://localhost:8123');
        this._webSocket.binaryType = 'arraybuffer';
        this._webSocket.onmessage = message => this._onMessage(message);

        return new Promise(resolve => this._webSocket.onopen = resolve);
    }

    _onMessage(message) {
        this._pendingMessages.push(new Uint8Array(message.data));
    }

    $sendMessage(bytes) {
        this._webSocket.send(bytes)
    }

    $readMessage() {
        let message = this._pendingMessages.pop();

        this._pendingMessages = [];

        return message;
    }

    $log(string) {
        console.log(string);
    }

    $logEnum(value) {
        console.log(value);
    }
}