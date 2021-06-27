import { WindowManager } from './window-manager';

export class Client {
    constructor(wasm) {
        this._wasm = wasm;
        this._webSocket = null;
        this._windowManager = new WindowManager();
        this._pendingMessages = [];
    }

    async start() {
        this._webSocket = new WebSocket('ws://localhost:8123');
        this._webSocket.binaryType = 'arraybuffer';
        this._webSocket.onmessage = message => this._onMessage(message);

        await this._windowManager.start();

        return new Promise(resolve => this._webSocket.onopen = resolve);
    }

    _onMessage(message) {
        this._pendingMessages.push(new Uint8Array(message.data));
    }

    $sendMessage(bytes) {
        this._webSocket.send(bytes)
    }

    $pollMessage() {
        let message = this._pendingMessages.pop();

        this._pendingMessages = [];

        return message;
    }

    $pollEvent() {
        let event = this._windowManager.pollEvent();

        if (event) {
            let { type, payload } = event;

            event = new this._wasm.Event();

            if (type === 'mouse') {
                payload = Object.assign(new this._wasm.MouseEvent(), payload);
            } else if (type === 'keyboard') {
                payload = Object.assign(new this._wasm.KeyboardEvent(), payload);
            }

            event[type] = payload;
        }

        return event;
    }

    $log(string) {
        console.log(string);
    }
}