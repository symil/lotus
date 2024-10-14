import { BufferWrapper } from './buffer-wrapper.js';

export class MemoryManager {
    constructor({ getMemory }) {
        this._getMemory = getMemory;
    }

    readBuffer(addr) {
        return new BufferWrapper(this._getMemory(), addr);
    }

    readString(addr) {
        let memory = this._getMemory();
        let length = memory[addr];
        let codes = new Array(length);

        for (let i = 0; i < length; ++i) {
            codes[i] = memory[addr + 2 + i];
        }

        return String.fromCodePoint(...codes);
    }
}