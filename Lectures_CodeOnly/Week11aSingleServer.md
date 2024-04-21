// Final Project: MultiThreaded Web Server

// Plan

// 1) TCP, HTTP
// 2) Listen for TCP connections
// 3) Parse HTTP requests
// 4) Create an HTTP response
// 5) Create a thread pool.


// Part #1 Building a Single-Threaded Web Server

=> The Basic Idea: An Event Loop.

"Canonical event-based server"

'''
while true {
    events = getEvents();
    for (e in events) {
        processEvent(e)
    }
}
'''

So here main actors:

Event -> (what kind of task needs to be done) (I/O request, network request,etc.)
getEvents() -> function which returns tasks need to be DONE in form of collection
processEvent() -> processing each event is known as *event handler*

Key idea: There will be only one event processed at a time.

 // Hypertext Transfer Protocol (HTTP) and Transmission Control Protocol (TCP). 

// Both protocols are *request-response protocols

//--> Client initiates requests 
//--> Server listens and response to the client.

// TCP responsible of how data passed around
// HTTP is actual data

// Logic #1 Listening to the TCP Connection

// // Code Section #1 START
// use std::net::TcpListener;

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
//     // you need to bind to 0.0.0.0 specific to repl, on your home machine you can use any address

//     // TCPListener is basically server which going to respond for any connections to the address we specified, those bind() is like new() method.


//     // Since creating of instance of server may fail, I will get back Result enum with variants <T,E>, where T is a generic representic correct result or E, which as an error. For that reason, I need to unwrap() it.
    
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         println!("Connection established")
//     }

//     // I hope this for loop reminds you the idea of event loop,
//     // every stream is basically a client which is connected to our server.

//     // another interesting thing to note, that use of variable overshadowing. so I overshadow stream with -> stream.unwrap()

//     // Conclusion: For now we are not doing anything with client, we just allow them to connect to our server.
//     // The fact we can trace with print statements.

//     // Client Request and Server Response -> is a single EVENT!

//     // From Rust book:  The reason we might receive errors from the incoming method when a client connects to the server is that we’re not actually iterating over connections. Instead, we’re iterating over connection attempts.
// }
// // Code Section #1 END


// Logic #2 Reading the Request -> first part of an event

// // Code Section #2 START


// use std::io::prelude::*; // import most common traits to work with I/O
// use std::io::BufReader; // BufferReader is a struct which gives us access for all methods we usually need to work with buffer
// // doc: https://doc.rust-lang.org/std/io/struct.BufReader.html
// use std::net::TcpListener; // server
// use std::net::TcpStream; // since we have a binding of read and write, we need to somehow treat single event (Request+ Response) for every client.
// // data between client and server passed through TCP protocol.
// // doc: https://doc.rust-lang.org/std/net/struct.TcpStream.html


// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         handle_connection(stream); // so our handle_connection method, now is responsible to react to a client request, whether it happen to be.
//     }
// }

// fn handle_connection(mut stream: TcpStream) { // here subtle thing that 
//     // by processing stream, we gonna intrinsically mutate it's state
//     // to make sure it's exhausted. because we will read the content of request into a buffer first before processing, so internal state of file descriptor, must keep which is a current byte we are reading.

//     let buf_reader = BufReader::new(&mut stream);// we are creating instance of buffer and passing into it our stream, and saying I am gonna ask you to read something in the future specifically from that stream.
//     // BufReader will read bytes, so he doesn't really care from where they come from , it can be a network connection, file, or just string.


//     // here code interesting because of method chaining.
//     // you can think you kind of create a data processing pipeline
    
//     let http_request: Vec<_> = buf_reader
//         .lines() // lines returns an iterator, when you call next it wil give you one line at a time.
//         .map(|result| result.unwrap()) // here you gonna apply for every element which next method gives unwrap(), by taking a reference to result(line), with the help of Fn Trait(same as &T reference)
        
//         .take_while(|line| !line.is_empty())// take_while method served the purpose to exhaust your iterator by checking if buf reader read the empty line, and basically you iterator should stop

//         // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.take_while
//         .collect(); 
//     // then you need to collect all element which are left inside of your data pipeline into VECTOR, which we defined Vec<_>, asking compiler to fill out the data type for us.


//     println!("Request: {:#?}", http_request); // printing content of vector in a debug mode :?

//     // -> content of request is a what our client wants server to do.

//     // GET -> client asking for data from the server
//     // POST -> client is sending data to the server

//     }


// // Code Section #2 END

// Logic #3 Writing Response -> second part of an event

// // Code Section #3 START

use std::io::prelude::*;
use std::io::BufReader; 
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream); 
    }
}

// // Empty page
// fn handle_connection(mut stream: TcpStream) { 

//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines() 
//         .map(|result| result.unwrap()) 
//         .take_while(|line| !line.is_empty())
//         .collect(); 

//     // response, just respond 200 OK, that you are good I heard you :)
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     stream.write_all(response.as_bytes()).unwrap();

//     // we need to convert our string to bytes first, before writing them back to the client, data traveled through TCP in the form of bytes only.
//     // to be precise in packets
//     //https://en.wikipedia.org/wiki/Transmission_Control_Protocol

//     // Note: Well just returning 200 is Ok but, we better send some data back to client.

//     }

// Send back a bytes for our html page. "hello.html"
// Hello World.

// content of hello.html file, which will be located in our server

// <!DOCTYPE html>
// <html lang="en">
//   <head>
//     <meta charset="utf-8">
//     <title>UTRGV!</title>
//   </head>
//   <body>
//     <h1>UTRGV To The Moon!</h1>
//     <p>Hi from Rust</p>
//   </body>
// </html>

// fn handle_connection(mut stream: TcpStream) { 

//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines() 
//         .map(|result| result.unwrap()) 
//         .take_while(|line| !line.is_empty())
//         .collect();


//     use std::fs;
//     // well since our html page techincally a file,
//     // we should open it first

//     let status_line = "HTTP/1.1 200 OK";

//     let contents = fs::read_to_string("hello.html").unwrap();

//     // here another subtle moment, which is easy to miss, we can read_to_string because a string struct, by itself is a Vec<u8>, so convenient

//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); // format macro helps us to create a final string before writing them to a stream in bytes.

//     stream.write_all(response.as_bytes()).unwrap();
// }

// well ignoring request, is kind of strange, we want to render our HTML page if we recieve GET respond specifically

// only respond to GET requests
// for all others 404.html for now
// <!DOCTYPE html>
// <html lang="en">
//   <head>
//     <meta charset="utf-8">
//     <title>Hello!</title>
//   </head>
//   <body>
//     <h1>Oops!</h1>
//     <p>Sorry, I don't know what you're asking for.</p>
//   </body>
// </html>


// fn handle_connection(mut stream: TcpStream) { 
//     use std::fs;
//     let buf_reader = BufReader::new(&mut stream);
//     // so from the header we only interested in first line, so we again obtain our iterator, but call next only once!
    
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     if request_line == "GET / HTTP/1.1" {
        
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = fs::read_to_string("hello.html").unwrap();
//         let length = contents.len();
//         let response =
//             format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); 
//         stream.write_all(response.as_bytes()).unwrap();
        
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND";
//         let contents = fs::read_to_string("404.html").unwrap();
//         let length = contents.len();

//         let response = format!(
//             "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//         );

//         stream.write_all(response.as_bytes()).unwrap();
//     }
// }

// Nice refactoring from the book just could not skip it
// I hope you will always try to go extra mile in your code.
// This is what usually distinguishes professional from amateur

// REFACTORING =>

fn handle_connection(mut stream: TcpStream) {
    use std::fs;
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // everything is expression in Rust.
    // next 4 lines is just superb.
    // Please notice no semicolon, you are returning those two str
    // and unpacking them immideately.

    let (status_line, filename ) = if request_line == "GET / HTTP/1.1"
        {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}

// // Code Section #3 END