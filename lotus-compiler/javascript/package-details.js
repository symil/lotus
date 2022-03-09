import fs from 'fs';
import path from 'path';
import toml from 'toml';
import { PACKAGE_CONFIG_FILE_NAME } from './constants';

const FRAMEWORKS = ['none', 'quick-game'];

const STRING_TYPE = { check: x => typeof x === 'string', name: 'string' };
const BOOL_TYPE = { check: x => typeof x === 'boolean', name: 'boolean' };

const FIELDS = {
    name: [STRING_TYPE, root => path.basename(root)],
    framework: [BOOL_TYPE, false],
    title: [STRING_TYPE, '']
}

export function readPackageDetails(packageRootPath) {
    let packageConfigFilePath = path.join(packageRootPath, PACKAGE_CONFIG_FILE_NAME);
    let result = {};

    if (fs.existsSync(packageConfigFilePath)) {
        let content = fs.readFileSync(packageConfigFilePath, 'utf-8');

        result = toml.parse(content);
    }

    let errors = formatConfiguration(result, packageRootPath);

    for (let error of errors) {
        console.error(`${PACKAGE_CONFIG_FILE_NAME}: ${error}`);
    }

    result.packageRootPath = packageRootPath;

    return result;
}

function formatConfiguration(config, packageRootPath) {
    let errors = [];

    for (let [fieldName, fieldEntry] of Object.entries(FIELDS)) {
        if (!Array.isArray(fieldEntry)) {
            fieldEntry = [fieldEntry, undefined];
        }

        let [ type, defaultValue ] = fieldEntry;
        let value = config[fieldName];

        if (Array.isArray(type)) {
            type = makeEnum(type);
        }

        if (value !== undefined && !type.check(value)) {
            errors.push(`field \`${fieldName}\`: expected ${type.name}, got ${JSON.stringify(value)}`);
            value = undefined;
        }

        if (value == undefined) {
            if (typeof defaultValue === 'function') {
                value = defaultValue(packageRootPath);
            } else {
                value = defaultValue;
            }
        }

        config[fieldName] = value;
    }

    return errors;
}

function makeEnum(values) {
    return {
        check: value => values.includes(value),
        name: `one of ${values.map(x => JSON.stringify(x)).join(', ')}`
    };
}