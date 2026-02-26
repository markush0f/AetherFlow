import { Request, Response, NextFunction } from 'express';
import { TranslatorService } from '../services/translatorService';

export class TranslateController {
    static async translateText(req: Request, res: Response, next: NextFunction): Promise<void> {
        try {
            const payload = req.body.payload || req.body;
            const textToTranslate: string = payload.text;
            const targetLanguage: string = payload.target_language || 'English';

            if (!textToTranslate) {
                res.status(400).json({ error: 'Missing "text" property in payload' });
                return;
            }

            const translation = await TranslatorService.translate(textToTranslate, targetLanguage);

            res.json({
                success: true,
                translation: translation
            });
        } catch (error) {
            next(error);
        }
    }
}
