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
            try {
                this._layout = await this._window.navigator.keyboard.getLayoutMap();
            } catch {}
        }

        return this;
    }

    getKeyValue(key) {
        if (this._layout) {
            return this._layout.get(key) || '';
        } else {
            return `Key${key}`;
        }
    }
}