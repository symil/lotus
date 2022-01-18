import chalk from 'chalk';
import { execSync } from 'child_process';
import { basename } from 'path';
import { OUTPUT_HTTP_SERVER_FILE_NAME, SSH_HOSTNAME, SSH_PORT, SSH_USER } from './constants';

export function uploadBuild(locations) {
    let targetDirName = basename(locations.rootDirPath);
    let preUploadCommandList = [
        `killall node`,
        `rm -rf ${targetDirName}`,
    ];
    let postUploadCommandList = [
        `cd ${targetDirName}`,
        `nohup node ${OUTPUT_HTTP_SERVER_FILE_NAME} > /dev/null 2>&1 &`
    ];

    let preUploadCommand = makeSshCommand(preUploadCommandList);
    let uploadCommand = makeScpUploadCommand(locations.buildDirPath, targetDirName);
    let postUploadCommand = makeSshCommand(postUploadCommandList);

    run(preUploadCommand, 'removing existing build');
    run(uploadCommand, 'uploading build');
    run(postUploadCommand, 'starting server');
}

export function stopRemoteServer(locations) {
    let stopCommand = makeSshCommand(['killall node']);

    run(stopCommand, 'stopping server');
}

function makeSshCommand(commandList) {
    return `ssh -p ${SSH_PORT} ${SSH_USER}@${SSH_HOSTNAME} '${commandList.join('; ')}'`;
}

function makeScpUploadCommand(localDirectoryPath, remoteDirectoryPath) {
    return `scp -P ${SSH_PORT} -q -r ${localDirectoryPath} ${SSH_USER}@${SSH_HOSTNAME}:/home/${SSH_USER}/${remoteDirectoryPath}`;
}

function run(command, logMessage) {
    process.stdout.write(chalk.bold.magenta(`> ${logMessage}...`));
    execSync(command);
    process.stdout.write(chalk.bold.magenta(' ok\n'));
}