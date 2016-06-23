// Express
const express = require('express');
const app = express();

app.get('/search', function (req, res) {
    res.send(req.query);
});

app.listen(parseInt(process.argv[2]));
