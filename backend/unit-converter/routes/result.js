const express = require('express');
const router = express.Router();

router.get('/', (req, res) => {
    const { value, fromUnit, toUnit, convertedValue } = req.query;
    res.render('converter', { value, fromUnit, toUnit, convertedValue });
});

module.exports = router;