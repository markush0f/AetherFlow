import { Ollama } from 'ollama';

const ollama = new Ollama({ host: 'http://127.0.0.1:11434' });

export class SummarizerService {
    static async summarize(text: string): Promise<string> {
        const prompt = `Please provide a concise summary of the following text. Do not include any introductory remarks, just the summary.\n\nText: "${text}"`;

        try {
            const response = await ollama.chat({
                model: 'llama3.2',
                messages: [{ role: 'user', content: prompt }],
            });

            return response.message.content.trim();
        } catch (error) {
            console.error('Error communicating with Ollama:', error);
            throw new Error('Summarization failed');
        }
    }
}
