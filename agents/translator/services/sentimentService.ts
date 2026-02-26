import { Ollama } from 'ollama';

const ollama = new Ollama({ host: 'http://127.0.0.1:11434' });

export class SentimentService {
    static async analyze(text: string): Promise<string> {
        const prompt = `Analyze the sentiment of the following text. Respond with only one word: POSITIVE, NEGATIVE, or NEUTRAL.\n\nText: "${text}"`;

        try {
            const response = await ollama.chat({
                model: 'llama3.2',
                messages: [{ role: 'user', content: prompt }],
            });

            return response.message.content.trim();
        } catch (error) {
            console.error('Error communicating with Ollama:', error);
            throw new Error('Sentiment analysis failed');
        }
    }
}
