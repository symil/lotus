export class KeyboardManager {
    constructor({ getWindow }) {
        this._window = getWindow?.();
        this._layout = null;
    }

    static async new(env) {
        return await new KeyboardManager(env).init();
    }

    async init() {
        if (this._window) {
            this._layout = await this._window.navigator.keyboard.getLayoutMap();
        }

        return this;
    }

    getKeyValue(key) {
        return this._layout.get(key) || '';
    }
}