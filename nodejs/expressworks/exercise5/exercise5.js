const bodyparser = require('body-parser');

// Express
const express = require('express');
const app = express();

// use stylus
const stylus = require('stylus');
app.use(stylus.middleware(process.argv[3]));

// static
app.use(express.static(process.argv[3]));

app.listen(parseInt(process.argv[2]));
