export class MemoryBuffer {
    constructor(memory, index) {
        this._memoryAsInt = new Int32Array(memory);
        this._memoryAsFloat = new Float32Array(memory);
        this._startIndex = index;
        this._currentIndex = index;
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

    write(value) {
        this._memoryAsInt[this._currentIndex] = value;
        this._currentIndex += 1;
    }

    writeFloat(value) {
        this._memoryAsFloat[this._currentIndex] = value;
        this._currentIndex += 1;
    }
}