extern crate lib;

use lib::utils::args::{parse_server, ArgsServer};
use lib::utils::auth::parse_secret;
use lib::utils::regex::check_message;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  loop {
    let size = stream.read(&mut buffer).unwrap();
    let buffer_to_str = String::from_utf8_lossy(&buffer[0..size]);
    let res_from_client = check_message(&buffer_to_str, &["SAY:", "LIST:", "KICK:", "LEAVE:"]).ok();

    if res_from_client.is_some() {
      stream.write(b"Is Valid\r\n").unwrap();
    } else {
      stream.write(b"Is NOT Valid\r\n").unwrap();
    }
  }
}

fn main() {
  let args: ArgsServer = parse_server("7272");
  let secret = parse_secret(&args.authfile);
  println!("Secret is {}", secret);

  let bind_addr = format!("127.0.0.1:{}", args.port);
  let listener = TcpListener::bind(bind_addr.to_string()).unwrap();
  println!("Server starting on {}", bind_addr);

  // accept connections and process them serially, ignore bad connections
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        thread::spawn(|| {
          handle_client(stream);
        });
      }
      Err(_e) => continue,
    }
  }
}
