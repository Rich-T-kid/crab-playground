// web server
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
//use std::thread::{self, JoinHandle, Thread};
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    

    let get = b"GET / HTTP/1.1";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = threadPool::new(8);
    pool.log();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream)

    }
}
pub struct Worker{
    id: i32,
    thread: thread::JoinHandle<()>
}
impl Worker {
    fn new(id:i32) ->Self{
         let thread = thread::spawn(|| {});

        Self{id,thread}
    }
    
}
pub struct threadPool{
    workers: Vec<Worker>
}
impl threadPool {
    fn new(count:usize) -> Self{
        // cant be 0
        assert!(count > 0);
        let mut workers = Vec::with_capacity(count);
        for i in 0..count{
            workers.push(Worker::new(i as i32));
        }
        Self {workers}
    }
    fn log(&self){
        for worker in self.workers.iter(){
            println!("worker id: {}",worker.id);
        }
    }
}
