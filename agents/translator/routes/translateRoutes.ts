import express from 'express';
import { TranslateController } from '../controllers/translateController';

const router = express.Router();
router.post('/translate', TranslateController.translateText);

export default router;
