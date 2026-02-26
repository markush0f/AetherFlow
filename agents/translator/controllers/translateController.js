const TranslatorService = require('../services/translatorService');

class TranslateController {
    static async translateText(req, res, next) {
        try {
            // AetherFlow webhook sends data in the root or via a payload property, 
            // depending on how you configured the sender. 
            // We assume AetherFlow forwards { "text": "...", "target_language": "..." }
            // or the gateway passes it as { "payload": { "text": "...", "target_language": "..." } }
            
            const payload = req.body.payload || req.body;
            const textToTranslate = payload.text;
            const targetLanguage = payload.target_language || 'English';

            if (!textToTranslate) {
                return res.status(400).json({ error: 'Missing "text" property in payload' });
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

module.exports = TranslateController;
