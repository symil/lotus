import assert from 'assert';
import path from 'path';
import fs from 'fs';
import { compileParser, OUTPUT_FILE_NAME, runTest, SRC_DIR_NAME, TEST_DIR } from './test-utils';

function main() {
    if (!compileParser()) {
        process.exit(1);
    }

    let validateOutput = process.argv.includes('--validate');
    let testDirList = fs.readdirSync(TEST_DIR).filter(dirName => fs.statSync(path.join(TEST_DIR, dirName)).isDirectory());
    let passedTestList = process.argv.filter(str => testDirList.includes(str));
    let testsToRun = passedTestList.length > 0 ? passedTestList : testDirList;

    describe('Lotus', () => {
        for (let dirName of testsToRun) {
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