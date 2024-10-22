export function toSnakeCase(string) {
    return string.replace(/[a-z][A-Z]/g, str => str[0] + '_' + str[1]).toLowerCase();
}

export function camelToKebabCase(string) {
    return string.replace(/[a-z][A-Z]/g, str => str[0] + '-' + str[1]).toLowerCase();
}

export function kebabToCamelCase(string) {
    return string.replace(/-\w/g, str => str[1].toUpperCase());
}

export function hashNumbers(a, b) {
    let h = ((a << 5) - a) + b;

    return h & h;
}

export function hashString(string) {
    let hash = 1;

    for (let i = 0; i < string.length; ++i) {
        let code = string.charCodeAt(i);

        hash = hashNumbers(hash, code);
    }

    return hash;
}

export function hashNumberList(list) {
    let hash = 0;

    for (let number of list) {
        hash = hashNumbers(hash, number);
    }

    return hash;
}

export function colorToString({ r, g, b, a }) {
    return `rgba(${r},${g},${b},${a/255})`;
}

export function colorToArray({ r, g, b, a }) {
    return [ r, g, b, a ];
}

export function colorToU32({ r, g, b, a }) {
    return (r << 24) + (g << 16) + (b << 8) + a;
}

export function stringToArray(string) {
    let array = new Array(string.length);

    for (let i = 0; i < string.length; ++i) {
        array[i] = string.charCodeAt(i);
    }

    return array;
}

const VALID_CHAR_RANGE_START = 33;
const VALID_CHAR_RANGE_END = 126;
const CHARACTER_COUNT = VALID_CHAR_RANGE_END - VALID_CHAR_RANGE_START + 1;
const L = Math.log(CHARACTER_COUNT);

export function encodeUint32ArrayToString(array) {
    let result = '';

    for (let n of array) {
        let required_count = Math.max(0, Math.ceil(Math.log(n) / L));

        result += required_count;

        for (let i = 0; i < required_count; ++i) {
            let c = n % CHARACTER_COUNT;
            result += String.fromCharCode(VALID_CHAR_RANGE_START + c);
            n /= CHARACTER_COUNT;
        }
    }

    return result;
}

export function decodeStringToUint32Array(string) {
    let result = [];
    let i = 0;

    while (i < string.length) {
        let required_count = +string[i];
        let end = i + required_count + 1;
        let power = 0;
        let n = 0;

        i += 1;

        while (i < end) {
            n += (string.charCodeAt(i) - VALID_CHAR_RANGE_START) * CHARACTER_COUNT ** power;
            i += 1;
            power += 1;
        }

        result.push(n);
    }

    return Uint32Array.from(result);
}

/**
 * 
 * @param {WebAssembly.Instance} instance 
 * @returns {any}
 */
export function getWasmExports(instance) {
    return instance.exports;
}