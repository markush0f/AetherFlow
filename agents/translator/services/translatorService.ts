import { Ollama } from 'ollama';

const ollama = new Ollama({ host: 'http://127.0.0.1:11434' });

export class TranslatorService {
    static async translate(text: string, targetLanguage: string): Promise<string> {
        const prompt = `Translate the following text into ${targetLanguage}. Only respond with the direct translation, without additional comments or explanations.\n\nText: "${text}"`;

        try {
            const response = await ollama.chat({
                model: 'llama3.2',
                messages: [{ role: 'user', content: prompt }],
            });

            return response.message.content.trim();
        } catch (error) {
            console.error('Error communicating with Ollama:', error);
            throw new Error('Translation failed');
        }
    }
}
