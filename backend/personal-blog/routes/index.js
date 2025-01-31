const express = require('express');
const isAuthenticated = require('../auth');
const router = express.Router();
const file = require('../file-handler');

/* GET home page. */
router.get('/', function(req, res, next) {
  const articles = file.readArticles();
  res.render('index', { articles: articles });
});

router.get('/admin', isAuthenticated, (req, res) => {
  const articles = file.readArticles();
  res.render('admin', { articles });
})



module.exports = router;
