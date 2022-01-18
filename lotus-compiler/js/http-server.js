import express from 'express';
import { spawn } from 'child_process';
import { FILES_DIR_NAME, DEFAULT_HTTP_PORT, NODE_MODULES_DIR_NAME, SERVER_FILE_NAME } from './constants';

const FORBIDDEN_DIRECTORIES = [ FILES_DIR_NAME, NODE_MODULES_DIR_NAME ];

function main() {
    let port = DEFAULT_HTTP_PORT;

    for (let arg of process.argv.slice(2)) {
        if (+arg) {
            port = +arg;
        }
    }

    let gameServer = spawn('node', [ SERVER_FILE_NAME ], { stdio: 'inherit' });
    // TODO: properly stop server when http server is stopped

    let httpServer = express();

    for (let forbiddenDirectory of FORBIDDEN_DIRECTORIES) {
        httpServer.all(`/${forbiddenDirectory}/*`, (req, res) => {
           res.status(403).send({
              message: 'Access Forbidden'
           });
        });
    }

    httpServer.use(express.static('.'));
    httpServer.listen(port, null);
    console.log(`info: listening on port ${port}`);
}

main();