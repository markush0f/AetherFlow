import createApp from './app';
import translateRoutes from './routes/translateRoutes';
import summarizeRoutes from './routes/summarizeRoutes';

// Create the different apps
const translatorApp = createApp(translateRoutes);
const summarizerApp = createApp(summarizeRoutes);

// Config ports
const TRANSLATOR_PORT = process.env.TRANSLATOR_PORT || 4000;
const SUMMARIZER_PORT = process.env.SUMMARIZER_PORT || 4001;

// Start Translator
translatorApp.listen(TRANSLATOR_PORT, () => {
    console.log(`Translator Agent running on http://127.0.0.1:${TRANSLATOR_PORT}`);
});

// Start Summarizer
summarizerApp.listen(SUMMARIZER_PORT, () => {
    console.log(`Summarizer Agent running on http://127.0.0.1:${SUMMARIZER_PORT}`);
});
