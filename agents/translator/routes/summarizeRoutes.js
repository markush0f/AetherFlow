const express = require('express');
const router = express.Router();
const SummarizeController = require('../controllers/summarizeController');

router.post('/summarize', SummarizeController.summarizeText);

module.exports = router;
