const express = require('express');
const router = express.Router();
const hardCodedAdmin = require('../config').hardCodedAdmin;


router.get('/login', (req, res) => {
  res.render('login')
});

router.post('/login', (req, res) => {
  const { email, password } = req.body;
  if (hardCodedAdmin.email === email && hardCodedAdmin.password === password) {
    req.session.isAuthenticated = true;
    res.redirect("/admin");
  } else {
    res.send("Invalid email or password");
  }
})

module.exports = router;