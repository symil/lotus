import { Renderer } from './renderer';
import { toSnakeCase } from './utils';
import { WindowManager } from './window-manager';

export class Client {
    constructor() {
        this._wasm = null;
        this._webSocket = null;
        this._windowManager = new WindowManager();
        this._renderer = new Renderer(this._windowManager);
        this._pendingMessages = [];
    }

    async start() {
        this._setupRustInterface();
        
        await this._loadWasm();
        await this._setupConnection();
        await this._windowManager.start();

        this._wasm.start();
        this._update();
    }

    async _loadWasm() {
        this._wasm = await import('../pkg/lotus_client.js');
    }

    async _setupConnection() {
        this._webSocket = new WebSocket('ws://localhost:8123');
        this._webSocket.binaryType = 'arraybuffer';
        this._webSocket.onmessage = message => this._onMessage(message);

        return new Promise(resolve => this._webSocket.onopen = resolve);
    }

    _update() {
        this._wasm.update();
        requestAnimationFrame(() => this._update());
    }

    _onMessage(message) {
        this._pendingMessages.push(new Uint8Array(message.data));
    }

    _setupRustInterface() {
        let self = this;
        let rustInterface = {};

        for (let name of Object.getOwnPropertyNames(Client.prototype)) {
            if (name.startsWith('$')) {
                let rustName = toSnakeCase(name.substring(1));

                rustInterface[rustName] = function() {
                    return self[name](...arguments);
                };
            }
        }

        Object.assign(window, rustInterface);
    }

    $log(string) {
        console.log(string);
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

    $sendMessage(bytes) {
        this._webSocket.send(bytes)
    }

    $pollMessage() {
        let message = this._pendingMessages.pop();

        this._pendingMessages = [];

        return message;
    }

    $setWindowAspectRatio(aspectRatio) {
        return this._windowManager.setAspectRatio(aspectRatio);
    }

    $getWindowWidth() {
        return this._windowManager.getWidth();
    }

    $getWindowHeight() {
        return this._windowManager.getHeight();
    }

    $getStringId(string) {
        return this._renderer.getStringId(string);
    }

    $clearCanvas() {
        return this._renderer.clear();
    }

    $draw(primitive) {
        return this._renderer.draw(primitive);
    }

    $clearRendererCache() {
        return this._renderer.clearCache();
    }
}