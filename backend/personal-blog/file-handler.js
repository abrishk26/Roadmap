const fs = require('fs');
const articlesFilePath = require('./config').articlesFilePath;

const readArticles = () => {
  if (fs.existsSync(articlesFilePath)) {
    const data = fs.readFileSync(articlesFilePath, 'utf-8');
    return JSON.parse(data);
  } else {
    fs.writeFileSync(articlesFilePath, JSON.stringify([], null, 2), 'utf-8');
    const data = fs.readFileSync(articlesFilePath, 'utf-8');
    return JSON.parse(data);
  }
};

const writeArticles = (articles) => {
  fs.writeFileSync(articlesFilePath, JSON.stringify(articles, null, 2), 'utf-8');
};

module.exports = {
  readArticles,
  writeArticles
};