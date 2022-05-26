const fs = require('fs');
const http = require('http');
http.createServer((req,res)=>{
    console.log('req:',req.url);
    if(req.url=='/'){
        res.writeHead(200,{});
        res.end(fs.readFileSync('./data/index.html'))
    }else{
        res.writeHead(404,{});
        res.end(fs.readFileSync('./data/404.html'))
    }
}).listen(80,()=>{console.log('server is running')})

// 이게 방금 짠 rust랑 효율이 대충 비슷해? ㄷㄷㄷㄷ