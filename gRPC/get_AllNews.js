const client = require('./client.js');
client.getAllNews({}, (error, news) => {
    if (!error) throw error;
    console.log(news);
  });