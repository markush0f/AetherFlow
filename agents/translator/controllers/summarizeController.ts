import { Request, Response, NextFunction } from 'express';
import { SummarizerService } from '../services/summarizerService';

export class SummarizeController {
    static async summarizeText(req: Request, res: Response, next: NextFunction): Promise<void> {
        try {
            const payload = req.body.payload || req.body;
            const textToSummarize: string = payload.text;

            if (!textToSummarize) {
                res.status(400).json({ error: 'Missing "text" property in payload' });
                return;
            }

            const summary = await SummarizerService.summarize(textToSummarize);

            res.json({
                success: true,
                summary: summary
            });
        } catch (error) {
            next(error);
        }
    }
}
