import express from 'express';
import { SummarizeController } from '../controllers/summarizeController';

const router = express.Router();
router.post('/summarize', SummarizeController.summarizeText);

export default router;
