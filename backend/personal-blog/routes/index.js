import express from 'express';
const router = express.Router();

const articles = [
  { title: "First Title", date: "22-32-32" },
  { title: "Second Title", date: "23-32-32" }
];
/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { articles: articles });
});

module.exports = router;
