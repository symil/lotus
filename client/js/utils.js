export function toSnakeCase(string) {
    return string.replace(/[a-z][A-Z]/g, str => str[0] + '_' + str[1]).toLowerCase();
}

export function hashNumbers(a, b) {
    let h = ((a << 5) - a) + b;

    return h & h;
}

export function hashString(string) {
    let hash = 0;

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