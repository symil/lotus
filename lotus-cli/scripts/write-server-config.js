#!/usr/bin/env node

import fs from 'fs';
import path from 'path';

function main() {
    let projectPath = process.argv[2];
    let outputPath = process.argv[3];

    let config = {
        name: path.basename(projectPath)
    };

    fs.writeFileSync(outputPath, JSON.stringify(config, null, '  '));
}

main();