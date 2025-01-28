const express = require("express");
const redis = require('redis');
const router = express.Router();

router.get("/", (req, res) => {
    // Load environment variable
    require('dotenv').config();

    // get api key and extract the query parameter from the request
    const key = process.env.API_KEY;
    const city = req.query.city;

    // create the redis client
    const redisClient = redis.createClient({
        url: process.env.REDIS_URL
    });

    // handle error when creating redis clinet
    redisClient.on('error', (err) => console.error("Redis Clinet Error:", err));

    // try to connect with the redis server
    redisClient.connect()
        .then(() => {
            console.log("Connect to redis server.");
        })
        .catch((err) => {
            console.log("Error:", err);
        })

    // construct the cache key to retrieve cached data
    const cacheKey = `weather:${city.toLowerCase()}`


    // try to get cached data
    redisClient.get(cacheKey)
        .then((cachedData) => {
            // return the cached data if it exist
            if (cachedData) {
                console.log("Returning cached data");
                res.render('weather', { data: JSON.parse(cachedData) });
            } else { // make the third party api request to retrieve the data
                console.log("No cached data found for the key:", cacheKey);
                const date = new Date();
                const formattedDate = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;

                const baseUrl = `https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/${city}/${formattedDate}`;
                const queryParam = {
                    key: key
                };

                const queryString = new URLSearchParams(queryParam).toString();
                const requestUrl = `${baseUrl}?${queryString}`;


                fetch(requestUrl)
                    .then((response) => {
                        response.json()
                            .then(data => {
                                redisClient.setEx(cacheKey, 3600, JSON.stringify(data))
                                    .then(() => {
                                        console.log("stored data to the redis server successfully");
                                    })
                                    .catch(err => {
                                        console.error("Error:", err);
                                    }
                                    );
                                console.log("Returning fresh data");
                                res.render('weather', { data })
                            })
                            .catch(err => {
                                console.error("Error:", err);
                            });
                    })
                    .catch(err => {
                        console.error("Error:", err);
                    });
            }
        })
        .catch(err => {
            console.error("Error retrieving data from Redis:", err);
        });
});



module.exports = router;