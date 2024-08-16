use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // define a tecp socket listener
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let mut active_requests = Arc::new(Mutex::new(0));
    for stream in listener.incoming() {
        let active_requests = Arc::clone(&active_requests);
        let stream = stream.unwrap();
        // spawing threads
        thread::spawn(move || {
            {
                let mut connection = active_requests.lock().unwrap();
                *connection += 1;
                if *connection >= 3 {
                    thread::sleep(Duration::from_secs(2));
                }
            }
            handle_connection(stream);

            {
                // we need to put it into brakets because of the lock
                // when a lock is acquired we can end it up in a loop when it is waiting
                // for the release
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let mut request_line = buf_reader.lines().next();
    // we now parsing the response and structure it according the stream recived
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index.html")),
        "GET /page1 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            (Some("HTTP/1.1 200 OK\r\n"), Some("page1.html"))
        }

        "GET /page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("page2.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let responce = format!(
        "{} Content-Length: {}\r\n\r\n{}",
        status_line.unwrap(),
        contents.len(),
        contents
    );

    stream.write_all(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}
