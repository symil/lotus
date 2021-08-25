export class WasmEnv {
    constructor({ log }) {
        this._log = log;
        this._instance = null;
    }

    init(instance) {
        this._instance = instance;
    }

    log(value) {
        this._log(value);
    }

    getMemory() {
        return new Uint32Array(this._instance.exports.memory.buffer);
    }
}