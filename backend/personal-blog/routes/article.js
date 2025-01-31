const express = require("express");
const router = express.Router();
const isAuthenticated = require('../auth');
const file = require('../file-handler');


router.get("/article/:id", (req, res) => {
  const articles = file.readArticles();
  const articleId = parseInt(req.params.id);
  const article = articles.find((a) => a.id === articleId);

  if (article) {
    res.render("article", { article });
  } else {
    res.status(404).send("Article not found");
  }
});

router.get('/new', isAuthenticated, (req, res) => {
  res.render('new-article');
});

router.get('/update/:id', isAuthenticated, (req, res) => {
  res.render('update-article', { article: { id: req.params.id } });
});

router.get('/delete/:id', isAuthenticated, (req, res) => {
  let articles = file.readArticles();
  const articleId = parseInt(req.params.id);
  
  const article = articles.find(a => a.id === articleId)
  const index = articles.indexOf(article);
  
  console.log(index);
  console.log(article);
  if (index > -1) {
    articles.pop(index);
    file.writeArticles(articles);
  }
  
  articles = file.readArticles();
  res.render('admin', { articles });
})

router.post('/new', isAuthenticated, (req, res) => {
  let articles = file.readArticles();
  let id = 1
  if (articles.length > 0) {
    id = articles[articles.length - 1].id + 1
  }
  const newArticle = {
    id,
    title: req.body.title,
    date: req.body.date,
    content: req.body.content
  };
  
  articles.push(newArticle);
  file.writeArticles(articles);
  articles = file.readArticles();
  res.render('admin', { articles })
});

router.post('/update/:id', isAuthenticated, (req, res) => {
  let articles = file.readArticles();
  const articleId = parseInt(req.params.id);
  console.log(articleId);
  const article = articles.find(a => a.id === articleId);
  
  if (article) {
    article.title = req.body.title;
    article.date = req.body.date;
    article.content = req.body.content;
  
    file.writeArticles(articles);
  } else {
    console.log('Article not found.');
  }
  
  articles = file.readArticles();
  res.render('admin', { articles });
})

module.exports = router;
