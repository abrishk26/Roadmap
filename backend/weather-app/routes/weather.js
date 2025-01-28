const express = require("express");
const router = express.Router();

router.get("/", (req, res) => {
    require('dotenv').config();
    const key = process.env.API_KEY;
    const city = req.query.city;

    const date = new Date();
    const formattedDate = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
    
    const baseUrl = `https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/${city}/${formattedDate}`;
    const queryParam = {
        key: key
    };

    const queryString = new URLSearchParams(queryParam).toString();
    const requestUrl = `${baseUrl}?${queryString}`;

    fetch(requestUrl)
        .then(response => response.json())
        .then(data => {
            // console.log('Response:', data);
            res.render('weather', { data })
        })
        .catch(error => {
            console.log('Error:', error);
        });
});

module.exports = router;