let path = require('path');
let express = require('express');

let app = express();

let PORT = process.env.PORT || '3000';

express.static.mime.types['wasm'] = 'application/wasm';

app.use(express.static(__dirname));
app.use(express.static(path.join(__dirname, 'target', 'wasm32-unknown-unknown', 'debug')));

app.listen(PORT, () => {
  console.log(`Listening on port ${PORT}.`);
});
