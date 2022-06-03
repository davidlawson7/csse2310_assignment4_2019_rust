extern crate lib;

use lib::utils::args::{parse_client, ArgsClient};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

fn main() {
  let args: ArgsClient = parse_client();

  let bind_addr = format!("127.0.0.1:{}", args.port);
  let stream = TcpStream::connect(bind_addr.to_string()).unwrap();

  let stream_clone = stream.try_clone().unwrap();
  let mut reader = BufReader::new(stream);
  let mut writer = BufWriter::new(stream_clone);
  let mut buffer = String::new();
  let mut response = String::new();

  let stdin = io::stdin();
  let mut stdout = io::stdout();

  loop {
    print!(">> ");
    stdout.flush().unwrap();

    stdin.read_line(&mut buffer).expect("Failed to read line");
    stdout.flush().unwrap();

    writer.write(buffer.as_bytes()).unwrap();
    writer.flush().unwrap();
    buffer.clear(); // truncate String buffer here
    reader.read_line(&mut response).unwrap();

    println!("{}", response.trim());
    response.clear();
  }
}
