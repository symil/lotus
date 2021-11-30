import { readStringFromBuffer } from './js-wasm-communication';

export class MemoryBuffer {
    constructor(memory, index, length) {
        this._memory = memory;
        this._memoryAsInt = new Int32Array(memory);
        this._memoryAsFloat = new Float32Array(memory);
        this._startIndex = index;
        this._currentIndex = index;
        this._length = length ?? Infinity;
    }

    isFinished() {
        return this._startIndex === 0 || this.getLength() >= this._length;
    }

    getLength() {
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

        if (addr) {
            return readStringFromBuffer(new MemoryBuffer(this._memory, addr));
        } else {
            return '';
        }
    }

    readColor() {
        let addr = this.read();
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
}