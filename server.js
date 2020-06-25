const express = require('express');
const app = express();
const port = 8080;

app.use(express.static('dist'));
app.get('/', (req, res) => res.redirect('index.html'));
app.listen(port, () => console.log(`\nListening at http://localhost:${port}`));