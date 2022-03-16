import path from 'path';
import fs from 'fs';
import readline from 'readline';
import { google } from 'googleapis';
import { GOOGLE_APIS_CREDENTIALS_PATH } from './constants';

// https://developers.google.com/sheets/api/quickstart/nodejs

const SCOPES = ['https://www.googleapis.com/auth/spreadsheets.readonly'];
const TOKEN_FILE_NAME = 'googleapis-token.json';
const DATA_DIR_NAME = 'data';

async function main() {
    let spreadsheetUrl = process.env.LOTUS_SPREADSHEET_URL;
    let cacheDirectoryPath = process.env.LOTUS_CACHE_DIR;
    let spreadsheetId = extractSpreadsheetId(spreadsheetUrl);
    let tokenPath = path.join(cacheDirectoryPath, TOKEN_FILE_NAME);
    let credentials = readJson(GOOGLE_APIS_CREDENTIALS_PATH);
    let auth = await setupClient(credentials, tokenPath);
    let data = await fetchSpreadsheetData(auth, spreadsheetId);

    writeSpreadsheetDataToCache(data, cacheDirectoryPath);
}

function writeSpreadsheetDataToCache(data, cacheDirectoryPath) {
    let dataDirPath = path.join(cacheDirectoryPath, DATA_DIR_NAME);

    fs.rmSync(dataDirPath, { recursive: true, force: true });
    fs.mkdirSync(dataDirPath, { recursive: true });

    for (let { title, rows } of data.sheets) {
        let filePath = path.join(dataDirPath, `${title}.tsv`);
        let content = rows.map(row => row.join('\t').replaceAll('\n', '\\n')).join('\n');

        fs.writeFileSync(filePath, content, 'utf8');
    }
}

async function fetchSpreadsheetData(auth, spreadsheetId) {
    let api = google.sheets({ version: 'v4', auth });
    let metadata = await api.spreadsheets.get({ spreadsheetId });
    let title = metadata.data.properties.title;
    let sheetTitles = metadata.data.sheets.map(sheet => sheet.properties.title);
    let ranges = sheetTitles;
    let dataRequest = await api.spreadsheets.values.batchGet({ spreadsheetId, ranges });
    let sheets = [];

    for (let i = 0; i < sheetTitles.length; ++i) {
        let title = sheetTitles[i];
        let rows = dataRequest.data.valueRanges[i].values;

        sheets.push({ title, rows });
    }

    return { title, sheets };
}

function extractSpreadsheetId(spreadsheetUrl) {
    let match = spreadsheetUrl.match(/^https?:\/\/docs\.google\.com\/spreadsheets\/d\/([\w-]+).*$/);
    let spreadsheetId = match?.[1];

    if (!spreadsheetId) {
        throw `Invalid spreadsheet URL.`;
    }

    return spreadsheetId;
}

function readJson(filePath) {
    let content = fs.readFileSync(filePath, 'utf8');
    let json = JSON.parse(content);

    return json;
}

async function setupClient(credentials, tokenPath) {
    let { client_secret, client_id, redirect_uris } = credentials.installed;
    let oAuth2Client = new google.auth.OAuth2(client_id, client_secret, redirect_uris[0]);

    if (!fs.existsSync(tokenPath)) {
        await generateNewToken(oAuth2Client, tokenPath);
    }

    oAuth2Client.setCredentials(readJson(tokenPath));

    return oAuth2Client;
}

async function generateNewToken(oAuth2Client, tokenPath) {
    return new Promise((resolve, reject) => {
        let authUrl = oAuth2Client.generateAuthUrl({
            access_type: 'offline',
            scope: SCOPES,
        });

        console.log('Authorize this app by visiting this url:', authUrl);

        let rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
        });

        rl.question('Enter the code from that page here: ', (code) => {
            rl.close();

            oAuth2Client.getToken(code, (err, token) => {
                if (err) {
                    console.error('Error while trying to retrieve access token', err);
                    reject(err);
                    return;
                }
                fs.mkdirSync(path.dirname(tokenPath), { recursive: true });
                fs.writeFileSync(tokenPath, JSON.stringify(token));
                resolve();
            });
        });
    });
}

main();