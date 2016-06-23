const path = require('path');
const bodyparser = require('body-parser');

// Express
const express = require('express');
const app = express();

// use urlencoded
app.use(bodyparser.urlencoded({extended: false}));

app.post('/form', function (req, res) {
    res.end(req.body.str.split('').reverse().join(''));
});

app.listen(parseInt(process.argv[2]));
