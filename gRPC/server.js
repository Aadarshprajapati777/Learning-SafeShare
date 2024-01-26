const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');
const protoPath = "./news.proto";


const options ={
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true
};


var packageDefinition = protoLoader.loadSync(protoPath, options);
var newsProto = grpc.loadPackageDefinition(packageDefinition);


const server = new grpc.Server();

let news = [
    { id: "1", title: "Note 1", body: "Content 1", postImage: "Post image 1" },
    { id: "2", title: "Note 2", body: "Content 2", postImage: "Post image 2" },
  ];

  server.addService(newsProto.NewsService.service, {
    getAllNews: (_, callback) => {
      callback(null, news);
    },
  });
  

  server.bindAsync(
    "0.0.0.0:50051",
    grpc.ServerCredentials.createInsecure(),
    (error, port) => {
      console.log("Server running at http://0.0.0.0:50051");
      server.start();
    }
  );