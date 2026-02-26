const { Ollama } = require('ollama');

// Initialize Ollama client (connecting to local ollama instance)
const ollama = new Ollama({ host: 'http://127.0.0.1:11434' });

class TranslatorService {
    static async translate(text, targetLanguage) {
        const prompt = `Translate the following text into ${targetLanguage}. Only respond with the direct translation, without additional comments or explanations.\n\nText: "${text}"`;

        try {
            const response = await ollama.chat({
                model: 'llama3.2', // you can change this to any model you prefer/have locally
                messages: [{ role: 'user', content: prompt }],
            });

            return response.message.content.trim();
        } catch (error) {
            console.error('Error communicating with Ollama:', error);
            throw new Error('Translation failed');
        }
    }
}

module.exports = TranslatorService;
