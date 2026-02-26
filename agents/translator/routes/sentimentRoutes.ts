import express from 'express';
import { SentimentController } from '../controllers/sentimentController';

const router = express.Router();
router.post('/sentiment', SentimentController.analyzeSentiment);

export default router;
