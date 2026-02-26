const SummarizerService = require('../services/summarizerService');

class SummarizeController {
    static async summarizeText(req, res, next) {
        try {
            const payload = req.body.payload || req.body;
            const textToSummarize = payload.text;

            if (!textToSummarize) {
                return res.status(400).json({ error: 'Missing "text" property in payload' });
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

module.exports = SummarizeController;
