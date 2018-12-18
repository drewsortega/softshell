var express = require('express');
var app = express();
var example_json = require('./example');
const morgan = require('morgan');

app.use(morgan('dev'));
app.get('/', function (req, res) {
    res.status(200).json(example_json);
})

app.listen(3001);