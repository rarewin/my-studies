fs = require('fs');

// Express
const express = require('express');
const app = express();

app.get('/books', function (req, res) {
    fs.readFile(process.argv[3], function (err, data) {
	res.json(JSON.parse(data));
    });
});

// official solution..
/*
 * fs.readFile(filename, function(e, data) {
 *   if (e) return res.sendStatus(500)
 *     try {
 *       books = JSON.parse(data)
 *     } catch (e) {
 *       res.sendStatus(500)
 *     }
 *     res.json(books)
 * })
 */

app.listen(parseInt(process.argv[2]));
