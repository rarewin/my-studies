const path = require('path');
const express = require('express');
const app = express();

// for jade
// app.set('views', path.join(__dirname, 'templates'));
app.set('views', process.argv[3]);
app.set('view engine', 'jade');

app.get('/home', function (req, res) {
    res.render('index', {date: new Date().toDateString()});
});

app.listen(parseInt(process.argv[2]));
