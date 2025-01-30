const express = require('express');
const performConversion = require('../utils'); // Ensure the function is correctly implemented
const router = express.Router();

router.post('/', (req, res) => {
    console.log('Received raw request body:', req.body); // Debugging

    const { value, fromUnit, toUnit, type } = req.body;

    // Validate that all required fields exist
    if (!value || !fromUnit || !toUnit || !type) {
        console.error('Missing fields:', { value, fromUnit, toUnit, type });
        return res.status(400).json({ error: 'All fields are required' });
    }

    try {
        const convertedValue = performConversion(value, fromUnit, toUnit, type);
        
        console.log('Converted Data:', {value, fromUnit, toUnit, convertedValue}); // Debugging

        res.redirect(`/result?value=${value}&fromUnit=${fromUnit}&toUnit=${toUnit}&convertedValue=${convertedValue}`);
    } catch (error) {
        console.error('Conversion error:', error.message);
        res.status(400).json({ error: error.message });
    }
});

module.exports = router;
