var express = require('express');
var app = express();

app.use('/public', express.static('public'));
app.get('/', function(req, res){
   res.send("Hello world!");
});

app.listen(3000);