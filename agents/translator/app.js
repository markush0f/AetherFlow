const express = require('express');
const app = express();
const translateRoutes = require('./routes/translateRoutes');

app.use(express.json());

// Routes
app.use('/api', translateRoutes);

// General Error Handler
app.use((err, req, res, next) => {
    console.error(err.stack);
    res.status(500).json({ error: 'Something broke!' });
});

module.exports = app;
