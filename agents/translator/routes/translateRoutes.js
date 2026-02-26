const express = require('express');
const router = express.Router();
const TranslateController = require('../controllers/translateController');

// Define our translator webhook path that AetherFlow will hit
router.post('/translate', TranslateController.translateText);

module.exports = router;
