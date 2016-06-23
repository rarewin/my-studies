// Express
const express = require('express');
const app = express();

// crypto
const crypto = require('crypto');

app.put('/message/:id', function (req, res) {
    res.end(crypto.createHash('sha1').update(new Date().toDateString() + req.params.id).digest('hex'));
});

app.listen(parseInt(process.argv[2]));
