import assert from 'assert';
import path from 'path';
import fs from 'fs';
import { compileParser, OUTPUT_FILE_NAME, runTest, SRC_DIR_NAME, TEST_DIR } from './test-utils';

function main() {
    if (!compileParser()) {
        process.exit(1);
    }

    let validateOutput = process.argv.slice('--validate');
    let testDirList = fs.readdirSync(TEST_DIR).filter(dirName => fs.statSync(path.join(TEST_DIR, dirName)).isDirectory());

    describe('Lotus', () => {
        for (let dirName of testDirList) {
            let testName = dirName;
            let dirPath = path.join(TEST_DIR, dirName);
            let sourcePath = path.join(dirPath, SRC_DIR_NAME);
            let expectedOutputPath = path.join(dirPath, OUTPUT_FILE_NAME);

            it(testName, async () => {
                let actualOutput = await runTest(sourcePath, dirPath);
                let expectedOutput = fs.readFileSync(expectedOutputPath, 'utf8');

                if (validateOutput) {
                    fs.writeFileSync(expectedOutputPath, actualOutput, 'utf8');
                    expectedOutput = actualOutput;
                }

                assert.strictEqual(actualOutput, expectedOutput);
            });
        }
    });
}

main();