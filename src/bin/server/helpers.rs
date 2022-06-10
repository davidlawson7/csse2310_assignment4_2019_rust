use lib::utils::communication;
use lib::utils::regex::Commands;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;

pub struct ArgsServer {
  pub authfile: String,
  pub port: u32,
}

pub fn server_receive(
  reader: &mut BufReader<TcpStream>,
  commands: &[&str],
) -> Result<(Commands, String), io::Error> {
  let (c, m) = communication::receive(reader, commands)?;
  println!("(client) {c}:{m}");
  return Ok((c, m));
}

pub fn server_send(writer: &mut BufWriter<TcpStream>, msg: &str) -> Result<(), io::Error> {
  print!("(server) {msg}");
  std::io::stdout().flush()?;
  return communication::send(writer, msg);
}

pub fn parse_server(default_port: &str) -> ArgsServer {
  let authfile: String = std::env::args().nth(1).expect("no authfile given");
  let possible_port: String = std::env::args().nth(2).unwrap_or(default_port.to_string());
  let port: u32 = possible_port
    .parse::<u32>()
    .ok()
    .expect(&format!("port must be a u32, received {}", possible_port));
  return ArgsServer { authfile, port };
}

pub struct Client {
  pub name: String,
  pub say: u32,
  pub kick: u32,
  pub list: u32,
  pub reader: BufReader<TcpStream>,
  pub writer: BufWriter<TcpStream>,
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
}

pub fn negotiate_with_client(stream: TcpStream, server_secret: &str) -> Result<Client, io::Error> {
  let mut client = Client::new(stream);
  // Do the auth challenge
  server_send(&mut client.writer, "AUTH:\n")?;
  let (_, client_secret) = server_receive(&mut client.reader, &["AUTH:"])?;
  if client_secret.trim() != server_secret.trim() {
    return Err(io::Error::new(
      io::ErrorKind::Other,
      "Client secret did not match servers",
    ));
  }
  server_send(&mut client.writer, "OK:\n")?;

  // Negotiate name
  loop {
    server_send(&mut client.writer, "WHO:\n")?;
    let (_, proposed_name) = server_receive(&mut client.reader, &["NAME:"])?;
    if proposed_name.chars().count() == 0 || proposed_name == "Fred" {
      server_send(&mut client.writer, "NAME_TAKEN:\n")?;
      continue;
    }
    client.name = proposed_name;
    server_send(&mut client.writer, "OK:\n")?;
    return Ok(client);
  }
}

pub fn process_leave(_c: Commands) {}

pub fn process_say(_c: Commands, _msg: &str) {}

pub fn process_kick(_c: Commands, _msg: &str) {}

pub fn process_list(_c: Commands) {}
