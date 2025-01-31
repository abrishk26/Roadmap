const path = require('path');

const config = {
  articlesFilePath: path.join(__dirname, "articles.json")
};

module.exports = Object.freeze(config);