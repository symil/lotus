import fs from 'fs';
import path from 'path';
import toml from 'toml';
import { DEFAULT_HTTP_PORT, PACKAGE_CONFIG_FILE_NAME } from './constants.js';
import { camelToKebabCase } from './utils.js';

const STRING_TYPE = { check: x => typeof x === 'string', name: 'string' };
const BOOL_TYPE = { check: x => typeof x === 'boolean', name: 'boolean' };
const POSITIVE_INT = { check: x => typeof x === 'number' && x >= 0 && x % 1 === 0, name: 'positive integer' };
const STRING_ARRAY = { check: x => Array.isArray(x) && x.every(item => typeof item === 'string' && item), name: 'string array' };

const FIELDS = {
    name: [STRING_TYPE, root => path.basename(root)],
    framework: [BOOL_TYPE, false],
    title: [STRING_TYPE, ''],
    port: [POSITIVE_INT, DEFAULT_HTTP_PORT],
    remote: [STRING_TYPE, ''],
    clientFiles: [STRING_ARRAY, []],
    spreadsheetUrl: [STRING_TYPE, '']
}

export function readPackageDetails(packageRootPath) {
    let packageConfigFilePath = path.join(packageRootPath, PACKAGE_CONFIG_FILE_NAME);
    /** @type {any} */
    let result = {};

    if (fs.existsSync(packageConfigFilePath)) {
        let content = fs.readFileSync(packageConfigFilePath, 'utf-8');

        result = toml.parse(content);
    }

    result = formatConfiguration(result, packageRootPath);

    for (let error of result._errors) {
        console.error(`${PACKAGE_CONFIG_FILE_NAME}: ${error}`);
    }

    result.packageRootPath = packageRootPath;

    return result;
}

function formatConfiguration(tomlConfig, packageRootPath) {
    let result = {};
    let errors = [];

    for (let [fieldName, fieldEntry] of Object.entries(FIELDS)) {
        if (!Array.isArray(fieldEntry)) {
            fieldEntry = [fieldEntry, undefined];
        }

        let tomlKey = camelToKebabCase(fieldName);
        let [ type, defaultValue ] = fieldEntry;
        let value = tomlConfig[tomlKey];

        if (Array.isArray(type)) {
            type = makeEnum(type);
        }

        if (value !== undefined && typeof type === 'object' && !type.check(value)) {
            errors.push(`field \`${tomlKey}\`: expected ${type.name}, got ${JSON.stringify(value)}`);
            value = undefined;
        }

        if (value == undefined) {
            if (typeof defaultValue === 'function') {
                value = defaultValue(packageRootPath);
            } else {
                value = defaultValue;
            }
        }

        result[fieldName] = value;
    }

    result._errors = errors;

    return result;
}

function makeEnum(values) {
    return {
        check: value => values.includes(value),
        name: `one of ${values.map(x => JSON.stringify(x)).join(', ')}`
    };
}