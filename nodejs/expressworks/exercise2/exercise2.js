const path = require('path');
const express = require('express');
const app = express();

app.listen(parseInt(process.argv[2]));
app.use(express.static(process.argv[3]) || path.join(__dirname, 'public'));
