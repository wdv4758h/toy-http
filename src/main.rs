#![feature(convert)]    // for String.as_str

use std::env;
use std::fs;            // fs::read_dir
use std::net::{TcpListener, TcpStream};
use std::io::Write;     // for stream.write



trait HTTPHandler {
    fn http_response(&self, stream: &TcpStream);
}


trait TCPServer {
    fn serve(&self);
    fn handler(&self, stream: &TcpStream);
}


struct HTTPServer {
    address: String,
}


impl TCPServer for HTTPServer {
    fn serve(&self) {
        let listener = TcpListener::bind(self.address.as_str()).unwrap();

        // accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // connection succeeded
                    self.handler(&stream)
                },
                Err(e) => {
                    // connection failed
                    println!("{}", e);
                }
            }
        }

        // close the socket server
        drop(listener);
    }

    fn handler(&self, stream: &TcpStream) {
        // [TODO] filter other request

        // if it's HTTP request
        self.http_response(&stream)
    }
}


impl HTTPHandler for HTTPServer {
    fn http_response(&self, mut stream: &TcpStream) {
        // [TODO]
        //
        // HTTP Method
        //
        //      GET_handler
        //

        println!("{:?}", stream);

        ////////////////////////////////////////
        // return files in current folder
        ////////////////////////////////////////

        // [TODO]
        // add file link & return file content
        // folder support

        let paths = fs::read_dir("./").unwrap();

        let mut data = format!("<html><h1>test</h1><body>");

        for path in paths {
            data = format!("{}<li>{}</li>", data, path.unwrap().path().display());
        }

        data = format!("{}</body></html>", data);
        data = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", data.len(), data);

        let _ = stream.write(data.as_bytes());
        let _ = stream.flush();
    }
}


fn main() {
    let address = env::args().skip(1).next().unwrap();
    let server = HTTPServer { address: address };
    server.serve();
}
