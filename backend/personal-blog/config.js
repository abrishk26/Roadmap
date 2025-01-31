const path = require('path');

const config = {
  articlesFilePath: path.join(__dirname, "articles.json")
};

const hardCodedAdmin = {
  email: "admin@example.com",
  password: "password123"
};

module.exports = Object.freeze({
  config: Object.freeze(config),
  hardCodedAdmin: Object.freeze(hardCodedAdmin)
});