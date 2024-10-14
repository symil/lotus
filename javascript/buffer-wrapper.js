const DATA_OFFSET = 1;
const SIZE_OFFSET = 2;
const CAPACITY_OFFSET = 3;

export class BufferWrapper {
    constructor(memory, addr) {
        this._memory = memory;
        this._memoryAsFloat = new Float32Array(memory.buffer);
        this._sizeAddr = addr + SIZE_OFFSET;
        this._dataIndex = memory[addr + DATA_OFFSET];
    }

    _grow(size) {
        let currentSize = this._memory[this._sizeAddr];
        let newSize = currentSize + size;

        this._memory[this._sizeAddr] = newSize;

        return this._dataIndex + currentSize;
    }

    writeString(string) {
        let index = this._grow(string.length + 1);

        this._memory[index] = string.length;

        for (let i = 0; i < string.length; ++i) {
            this._memory[index + 1 + i] = string.charCodeAt(i);
        }
    }
}