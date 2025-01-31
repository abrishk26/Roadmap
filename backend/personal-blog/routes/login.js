const express = require('express');
const router = express.Router();

const hardCodedAdmin = {
  email: "admin@example.com",
  password: "password123"
};

// router.get('/', (req, res) => {
//   res.render('login')
// });

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