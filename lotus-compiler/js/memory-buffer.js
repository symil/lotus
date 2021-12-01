export class MemoryBuffer {
    constructor(memory, index, capacity) {
        this._memoryAsInt = memory;
        this._memoryAsFloat = new Float32Array(memory.buffer);
        this._startIndex = index;
        this._currentIndex = index;
        this._capacity = capacity ?? Infinity;
    }

    isFinished() {
        return this._startIndex === 0 || this.getLength() >= this._capacity;
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
            return readStringFromMemory(this._memoryAsInt, addr);
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

    writeBuffer(arrayBuffer) {
        let buffer = new Int32Array(arrayBuffer);

        this.write(buffer.length);
        this._memoryAsInt.set(buffer, this._currentIndex);
        this._currentIndex += buffer.length;
    }
}