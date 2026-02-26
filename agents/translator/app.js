const express = require('express');

function createApp(routes) {
    const app = express();
    app.use(express.json());

    // Routes
    app.use('/api', routes);

    // General Error Handler
    app.use((err, req, res, next) => {
        console.error(err.stack);
        res.status(500).json({ error: 'Something broke!' });
    });

    return app;
}

module.exports = createApp;
