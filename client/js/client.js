import { Renderer } from './renderer';
import { toSnakeCase } from './utils';
import { WindowManager } from './window-manager';

export class Client {
    constructor(wasm) {
        this._wasmPromise = wasm;
        this._wasm = null;
        this._webSocket = null;
        this._windowManager = new WindowManager();
        this._renderer = new Renderer(this._windowManager);
        this._pendingMessages = [];
    }

    static async start(wasm) {
        return await new Client(wasm).start();
    }

    async start() {
        this._setupRustInterface();

        await this._setupWasm();
        await this._setupConnection();
        await this._windowManager.start();

        this._wasm.start();
        this._update();

        return this;
    }

    async _setupWasm() {
        this._wasm = await this._wasmPromise;
    }

    async _setupConnection() {
        let host = this._wasm.get_host();

        this._webSocket = new WebSocket(`ws://${host}`);
        this._webSocket.binaryType = 'arraybuffer';
        this._webSocket.onmessage = message => this._onMessage(message);
        this._webSocket.onerror = error => console.error(error);

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
                let rustName = name.substring(1);

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

    $poll_event() {
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

    $send_message(bytes) {
        this._webSocket.send(bytes)
    }

    $poll_message() {
        let message = this._pendingMessages.pop();

        this._pendingMessages = [];

        return message;
    }

    $set_window_aspect_ratio(aspectRatio) {
        return this._windowManager.setAspectRatio(aspectRatio);
    }

    $get_window_width() {
        return this._windowManager.getWidth();
    }

    $get_window_height() {
        return this._windowManager.getHeight();
    }

    $set_window_title(title) {
        return this._windowManager.setTitle(title);
    }

    $get_string_id(string) {
        return this._renderer.getStringId(string);
    }

    $clear_canvas() {
        return this._renderer.clear();
    }

    $draw(primitive) {
        return this._renderer.draw(primitive);
    }

    $set_cursor(cursor) {
        return this._renderer.setCursor(cursor);
    }

    $clear_renderer_cache() {
        return this._renderer.clearCache();
    }
}