use tp_htp::ThreadPool;
use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*},
    thread,
    time::Duration,
};

fn main() {
    // println!("Hello, world!");
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // Debugging (This should execute once no matter how many browser incoming requests)
    println!("4 threads created should only be once");

    for stream in listner.incoming().take(2) {
        let stream = stream.unwrap();
        // println!("Connection established {:?}" ,stream);
        // Stram value is connection to the browser that's why we namr handle_connection paramter a "TcpStream"
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        pool.execute(|| {
            // Debugging
            println!("handle connection being executed");
            handle_connection(stream);
        });
        
    }  


    // fn handle_connection<T>(x: T)  {
    fn handle_connection(mut streamVal: TcpStream) {
        let buf_reader = BufReader::new(&streamVal);
        // let http_req : Vec<_> = buf_reader.lines();
        // let http_req: Vec<_>  = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
        let get_http = buf_reader.lines().next().unwrap().unwrap();
        println!("This is the val of get_http in buf reader {:?}", get_http);

        // let response = "HTTP/1.1 200 OK\r\n\r\n";
        // let (filename, status_line) = if get_http == "GET / HTTP/1.1" {
            
        //     ("index.html", "HTTP/1.1 200 OK")
        // }else {
        //     ("index2.html", "HTTP/1.1 404 Not Found")
        // };

        let (filename, status_line) = match get_http.as_str() {
            "GET / HTTP/1.1" => {
                ("index.html", "HTTP/1.1 200 OK")
            },
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("index.html", "HTTP/1.1 200 OK")
            },
            _ => ("index2.html", "HTTP/1.1 404 Not Found" )
        };

        let content = fs::read_to_string(filename).unwrap();
        let length = content.len();

        let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{content}");

        streamVal.write(response.as_bytes()).unwrap();

        // if get_http == "GET / HTTP/1.1" {

        //     let status_line = "HTTP/1.1 200 OK";
        //     let contents = fs::read_to_string("index.html").unwrap();
        //     let length = contents.len();
    
        //     let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
        //     // println!("This is the response {}", response);
        //     println!("Inside the IF statement");
        //     streamVal.write(response.as_bytes()).unwrap();
        // }
        // else{
        //     let status_line = "HTTP/1.1 404 Not Found";
        //     let contents = fs::read_to_string("index2.html").unwrap();
        //     let length = contents.len();

        //     let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
        //     println!("Inside the ELSE statement");
        //     streamVal.write(response.as_bytes()).unwrap();
        // }
    }
}

/* listener.incoming()
        |
        v
Main Thread owns stream
        |
        v
move closure owns stream
        |
        v
execute owns closure
        |
        v
Job owns closure
        |
        v
Channel Queue owns Job
        |
        v
Worker receives Job
        |
        v
job()
        |
        v
handle_connection(stream)
        |
        v
stream dropped
 */