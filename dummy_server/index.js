var express = require('express');
var app = express();
var example_json = require('./example');
var cors = require('cors');
var morgan = require('morgan');
app.use(cors());
app.use(morgan('dev'));

app.get('/api/twitter', function (req, res) {
    res.status(200).json(example_json);
})

app.listen(8000);