const express = require("express");
const router = express.Router();

router.get("/", (req, res) => {
    require('dotenv').config();
    console.log(process.env.API_KEY);
    const city = req.query.city;
    console.log(city);
    res.render('weather')
});

module.exports = router;