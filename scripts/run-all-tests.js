import assert from 'assert';
import path from 'path';
import fs from 'fs';
import { compileParser, runTest, SOURCE_EXTENSION, TEST_DIR } from './test-utils';

function main() {
    if (!compileParser()) {
        process.exit(1);
    }

    let testDirList = fs.readdirSync(TEST_DIR).filter(dirName => fs.statSync(path.join(TEST_DIR, dirName)).isDirectory());

    describe('Lotus', () => {
        for (let dirName of testDirList) {
            let testName = dirName;
            let dirPath = path.join(TEST_DIR, dirName);
            let sourcePath = path.join(dirPath, `${dirName}${SOURCE_EXTENSION}`);
            let expectedOutputPath = sourcePath.replace(SOURCE_EXTENSION, '.txt');

            it(testName, async () => {
                let actualOutput = await runTest(sourcePath, dirPath);
                let expectedOutput = fs.readFileSync(expectedOutputPath, 'utf8');

                assert.strictEqual(actualOutput, expectedOutput);
            });
        }
    });
}

main();