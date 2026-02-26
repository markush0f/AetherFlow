import express, { Express, Request, Response, NextFunction } from 'express';

export default function createApp(routes: express.Router): Express {
    const app = express();
    app.use(express.json());

    // Routes
    app.use('/api', routes);

    // General Error Handler
    app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
        console.error(err.stack);
        res.status(500).json({ error: 'Something broke!' });
    });

    return app;
}
