import { Request, Response, NextFunction } from 'express';
import { SentimentService } from '../services/sentimentService';

export class SentimentController {
    static async analyzeSentiment(req: Request, res: Response, next: NextFunction): Promise<void> {
        try {
            const payload = req.body.payload || req.body;
            const textToAnalyze: string = payload.text;

            if (!textToAnalyze) {
                res.status(400).json({ error: 'Missing "text" property in payload' });
                return;
            }

            const sentiment = await SentimentService.analyze(textToAnalyze);

            res.json({
                success: true,
                sentiment: sentiment
            });
        } catch (error) {
            next(error);
        }
    }
}
