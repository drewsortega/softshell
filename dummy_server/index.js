var express = require('express');
var app = express();
var example_json = require('./example');

app.get('/api/twitter', function (req, res) {
    res.status(200).json(example_json);
})

app.listen(8000);