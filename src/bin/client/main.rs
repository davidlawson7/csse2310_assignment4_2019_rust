mod helpers;

use lib::utils::auth::parse_secret;
use lib::utils::regex::ClientCommands;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  client().expect("failed to start client");
}

fn client() -> Result<&'static str, Error> {
  let args: helpers::ArgsClient = helpers::parse_client();
  let secret = parse_secret(&args.authfile);

  let bind_addr = format!("127.0.0.1:{}", args.port);
  let stream = TcpStream::connect(bind_addr.to_string()).unwrap();
  let stream_clone = stream.try_clone().unwrap();
  let mut reader = Arc::new(Mutex::new(BufReader::new(stream)));
  let mut writer = Arc::new(Mutex::new(BufWriter::new(stream_clone)));
  let mut writter_clone = Arc::clone(&writer);

  let _name = negotiate_with_server(&mut reader, &mut writer, &secret)?;

  let _handler = thread::spawn(move || {
    let sleep_time = std::time::Duration::from_millis(100);

    loop {
      let incoming = helpers::receive(&mut reader, &["ENTER", "LEAVE", "MSG", "KICK", "LIST"]);
      match incoming {
        Ok((_command, msg)) => {
          println!("{}", msg);
          thread::sleep(sleep_time);
        }
        Err(_err) => {
          println!("READER: server terminated connection");
          break;
        }
      }
    }
  });

  // In the current thread, handle outgoing traffic from the client
  loop {
    let input = helpers::read_stdin().unwrap();
    helpers::send(&mut writter_clone, &input).unwrap();
  }
}

/// Negotiate with the chat server a position in the server. Returns the determined name on the server if successful, else propogates a error upwards.
pub fn negotiate_with_server(
  reader: &mut Arc<Mutex<BufReader<TcpStream>>>,
  writer: &mut Arc<Mutex<BufWriter<TcpStream>>>,
  secret: &str,
) -> Result<String, Error> {
  println!("Waitng for AUTH prompt");
  let (_, _) = helpers::receive(reader, &["AUTH:"])?;
  println!("Received AUTH prompt");
  helpers::send(writer, secret)?;
  println!("Sent AUTH");
  let (_, _) = helpers::receive(reader, &["OK:"])?;
  println!("Received OK prompt");

  loop {
    println!("Doing the WHO");
    let (_, _) = helpers::receive(reader, &["WHO:"])?;
    let input = helpers::read_stdin()?;
    helpers::send(writer, &input)?;
    let (command, _) = helpers::receive(reader, &["NAME_TAKEN:", "OK:"])?;

    match command {
      ClientCommands::OK => {
        // client.set_name(input);
        return Ok(input);
      }
      ClientCommands::NAMETAKEN => {
        continue;
      }
      _ => {
        return Err(Error::new(ErrorKind::Other, "Negotiations broke down..."));
      }
    }
  }
}
