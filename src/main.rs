use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead}; 
use std::io::prelude;  
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap(); 
    let stream = listener.accept(); 

    println!("The stream {:?} \n The socket {:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0); 
}