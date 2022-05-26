// cd p_20_server
// https://rinthel.github.io/rust-lang-book-ko/ch20-01-single-threaded.html

use p_20_server::ThreadPool;

use std::io::prelude::*;
use std::mem::take;
//stream.read 위함
use std::net::{TcpListener, TcpStream};

use std::fs::File;


// 시간차
use std::thread;
use std::time::Duration;

fn main() {
    server();
}

fn server(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);


    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            req_handler(stream);
        });
        //req_handler(stream);
        
        //println!("Connection established!");
 
    }
    println!("Shutting down.");
}



fn req_handler(mut stream:TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    
    // let st = &*String::from_utf8_lossy(&buffer);
    let st = String::from_utf8_lossy(&buffer).to_string();
    // let splited:Vec<&str>  = st.split("\r\n").collect();
    // println!("Req: {}",splited[0]);
    println!("Req origin: {}",st);

    // 판단.
    let get = b"GET / HTTP/1.1\r\n"; 
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };


    // thread::sleep(Duration::from_secs(1));

    let mut file = File::open(format!("./data/{}", filename)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();




    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}