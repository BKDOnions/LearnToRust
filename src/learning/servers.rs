use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub fn single_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed To Bind Port");
    for req in listener.incoming() {
        let req = req.expect("Failed To Handle The Request");
        println!("Connection Successful");
        req_handler(req);
    }
}

fn req_handler(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        stream.read(&mut buffer).expect("R");
        println!("Req Content: {}", String::from_utf8_lossy(&buffer));
    }

    // let mut buffer = String::new();
    // stream.read_to_string(&mut buffer);
    // println!("Req Content: {}", buffer);
}

#[cfg(test)]
mod single_server_tests {
    use crate::learning::servers::single_server;

    #[test]
    fn single_server_test() {
        single_server();
    }
}
