import { readStringFromMemory } from './js-wasm-communication';

export class MemoryBuffer {
    constructor(memory, index, capacity) {
        this._memoryAsInt = memory;
        this._memoryAsFloat = new Float32Array(memory.buffer);
        this._startIndex = index;
        this._currentIndex = index;
        this._capacity = capacity ?? Infinity;
    }

    toRegularBuffer() {
        return Buffer.from(this._memoryAsInt.buffer, this._startIndex * 4, this._capacity * 4);
    }

    isFinished() {
        return this._startIndex === 0 || this.getSize() >= this._capacity;
    }

    getSize() {
        return this._currentIndex - this._startIndex;
    }

    read() {
        return this._memoryAsInt[this._currentIndex++];
    }

    readFloat() {
        return this._memoryAsFloat[this._currentIndex++];
    }

    readString() {
        let addr = this.read();

        if (!addr) {
            return '';
        }

        return readStringFromMemory(this._memoryAsInt, addr);
    }

    readColor() {
        let addr = this.read();

        if (!addr) {
            return { r: 0, g: 0, b: 0, a: 0 };
        }

        let r = this._memoryAsInt[addr + 1];
        let g = this._memoryAsInt[addr + 2];
        let b = this._memoryAsInt[addr + 3];
        let a = this._memoryAsInt[addr + 4];

        return { r, g, b, a };
    }

    readEnum(values) {
        return values[this.read()] || null;
    }

    write(value) {
        this._memoryAsInt[this._currentIndex] = value;
        this._currentIndex += 1;
    }

    writeFloat(value) {
        this._memoryAsFloat[this._currentIndex] = value;
        this._currentIndex += 1;
    }

    writeEnum(value, valueList) {
        this.write(valueList.indexOf(value));
    }

    /**
     * 
     * @param {Buffer|Uint8Array} buffer 
     */
    writeBuffer(buffer) {
        if (!buffer) {
            this.write(0);
            return;
        }
        
        let int32Array = new Int32Array(buffer.buffer, buffer.byteOffset, buffer.length / Int32Array.BYTES_PER_ELEMENT);

        this.write(int32Array.length);
        this._memoryAsInt.set(int32Array, this._currentIndex);
        this._currentIndex += int32Array.length;
    }
}