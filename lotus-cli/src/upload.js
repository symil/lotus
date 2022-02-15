import chalk from 'chalk';
import { execSync } from 'child_process';
import { basename } from 'path';
import { SSH_HOSTNAME, SSH_PORT, SSH_USER, START_SCRIPT_NAME } from './constants';

export function uploadBuild(locations) {
    let targetDirName = getTargetDirName(locations);
    let preUploadCommandList = [
        `rm -rf ${targetDirName}`,
    ];
    let postUploadCommandList = [
        `cd ${targetDirName} && ./${START_SCRIPT_NAME}`
    ];

    let sshPreUploadCommand = makeSshCommand(preUploadCommandList);
    let sshUploadCommand = makeScpUploadCommand(locations.buildDirPath, targetDirName);
    let sshPostUploadCommand = makeSshCommand(postUploadCommandList);

    run(sshPreUploadCommand, 'removing existing build');
    run(sshUploadCommand, 'uploading build');
    run(sshPostUploadCommand, 'starting server');
}

export function stopRemoteServer(locations) {
    let targetDirName = getTargetDirName(locations);
    let commands = [
        `cd ${targetDirName} && ./${START_SCRIPT_NAME} --stop`
    ];
    let sshStopCommand = makeSshCommand(commands);

    run(sshStopCommand, 'stopping server');
}

function getTargetDirName(locations) {
    return basename(locations.rootDirPath);
}

function makeSshCommand(commandList) {
    if (!Array.isArray(commandList)) {
        commandList = [commandList];
    }

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