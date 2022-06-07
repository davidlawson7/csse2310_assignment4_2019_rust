use lib::utils::regex::check_message;
use lib::utils::regex::ClientCommands;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

pub struct Client {
  pub name: String,
  pub say: u32,
  pub kick: u32,
  pub list: u32,
  reader: BufReader<TcpStream>,
  writer: BufWriter<TcpStream>,
}

impl Client {
  pub fn new(stream: TcpStream) -> Client {
    let stream_clone = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream_clone);

    Client {
      name: "default".to_string(),
      say: 0,
      kick: 0,
      list: 0,
      reader,
      writer,
    }
  }

  pub fn send(&mut self, msg: &str) {
    self.writer.write(msg.as_bytes()).unwrap();
    self.writer.flush().unwrap();
  }

  pub fn receive(&mut self, commands: &[&str]) -> Result<(ClientCommands, String), &'static str> {
    let mut buffer = String::new();
    let incoming = self.reader.read_line(&mut buffer);
    match incoming {
      Ok(_size) => {
        let res = check_message(&buffer, commands).ok();

        if res.is_some() {
          let (command, msg) = res.unwrap();
          return Ok((command, msg.to_string()));
        }
        return Err("Failed to process the message correctly");
      }
      Err(_err) => {
        return Err("Failed to receive the message correctly... Message: {}");
      }
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
}

// let res_from_client = check_message(&buffer, &["AUTH:"]).ok();
// let (_command, secret) = res_from_client.unwrap();
// println!("Secret from client: {}", secret);

// println!("Failed to negotiate with the client");
