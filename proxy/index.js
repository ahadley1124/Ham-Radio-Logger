const http = require('http');
const https = require('https');
const xml2js = require('xml2js');
const nodecache = require('node-cache');

const PORT = 3190;
const cache = new nodecache({ stdTTL: 86400, checkperiod: 10800 });

const url = 'https://xmldata.qrz.com/xml/current/?username=ke8ygw;password=AAUUSStinhh1124*;agent=testke8ygw';

// deepcode ignore HttpToHttps: HTTPS truncated upstream
const server = http.createServer((req, res) => {
    if (req.url === '/login') {
        https.get(url, (response) => {
            let data = '';
            response.on('data', (chunk) => {
                data += chunk;
            });
            response.on('end', () => {
                xml2js.parseString(data, (err, result) => {
                    if (err) {
                        console.error(err);
                        res.writeHead(500, { 'Content-Type': 'text/plain' });
                        res.end('500 Internal Server Error');
                    } else {
                        res.writeHead(200, { 'Content-Type': 'application/json' });
                        res.end(result.QRZDatabase.Session[0].Key[0]);
                    }
                });
            });
        }).on('error', (err) => {
            console.error(err);
            res.writeHead(500, { 'Content-Type': 'text/plain' });
            res.end('500 Internal Server Error');
        });
    } else if (req.url.startsWith('/callsign?')) {
        const searchParams = new URL(req.url, `http://${req.headers.host}`).searchParams;
        let callsign = searchParams.get('callsign');
        console.log(`Received request for callsign ${callsign}`);
        const cachedResult = cache.get(callsign);
        if (cachedResult) {
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(cachedResult);
        } else {
            res.writeHead(200, { 'Content-Type': 'application/json' });
            // send a json encoded string that the cache doesn't have the callsign
            const string = `${callsign} is not in the cache. Please request it from QRZ.com`;
            res.end(JSON.stringify({ string }));
        }
    } else if (req.url === '/cache') {
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });
        req.on('end', () => {
            try {
                const { callsign, result } = JSON.parse(body);
                cache.set(callsign, result);
                res.writeHead(200, { 'Content-Type': 'text/plain' });
                res.end(`Added ${callsign} to cache`);
            } catch (error) {
                console.error(error);
                res.writeHead(400, { 'Content-Type': 'text/plain' });
                res.end('400 Bad Request');
            }
        });
    }
});

server.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
