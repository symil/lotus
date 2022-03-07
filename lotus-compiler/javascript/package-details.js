import fs from 'fs';
import path from 'path';
import toml from 'toml';
import { PACKAGE_CONFIG_FILE_NAME } from './constants';

const FIELDS = {
    name: ['string', root => path.basename(root)]
}

const TYPES = {
    string: x => typeof x === 'string'
};

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

        let [ typeName, defaultValue ] = fieldEntry;
        let checkFunction = TYPES[typeName];
        let value = config[fieldName];

        if (value !== undefined && !checkFunction(value)) {
            errors.push(`field \`${fieldName}\`: expected ${typeName}, got ${JSON.stringify(value)}`);
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