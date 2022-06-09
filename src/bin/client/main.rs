mod helpers;

use crate::helpers::parse_client;
use lib::utils::auth::parse_secret;
use lib::utils::communication;
use lib::utils::regex::Commands;
use std::io::Write;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::net::TcpStream;
// use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  client().expect("failed to start client");
}

fn client() -> Result<&'static str, Error> {
  let args: helpers::ArgsClient = parse_client();
  let secret = parse_secret(&args.authfile);

  let bind_addr = format!("127.0.0.1:{}", args.port);
  let stream = TcpStream::connect(bind_addr.to_string()).unwrap();
  let stream_clone = stream.try_clone().unwrap();
  // let mut reader = Arc::new(Mutex::new(BufReader::new(stream)));
  // let mut writer = Arc::new(Mutex::new(BufWriter::new(stream_clone)));
  // let mut writter_clone = Arc::clone(&writer);
  let mut reader = BufReader::new(stream);
  let mut writer = BufWriter::new(stream_clone);

  let _name = negotiate_with_server(&mut reader, &mut writer, &secret)?;

  thread::spawn(move || loop {
    let incoming = communication::receive(&mut reader, &["ENTER", "LEAVE", "MSG", "KICK", "LIST"]);
    match incoming {
      Ok((command, msg)) => {
        println!("READY FOR PROCESSING => {command}:{msg}");
        match command {
          Commands::ENTER => helpers::process_enter(command, &msg),
          Commands::LEAVE => helpers::process_leave(command, &msg),
          Commands::MSG => helpers::process_msg(command, &msg),
          Commands::KICK => helpers::process_kick(command),
          Commands::LIST => helpers::process_list(command, &msg),
          _ => println!("Receieved a message the client shouldnt, {command} - {msg}."),
        }
      }
      Err(_err) => {
        println!("READER: server terminated connection");
        break;
      }
    }
  });

  // In the current thread, handle outgoing traffic from the client
  let sleep_time = std::time::Duration::from_millis(100);
  loop {
    println!("ready for input");
    let input = helpers::read_stdin().unwrap();
    communication::send(&mut writer, &input).unwrap();
    thread::sleep(sleep_time);
  }
}

/// Negotiate with the chat server a position in the server. Returns the determined name on the server if successful, else propogates a error upwards.
pub fn negotiate_with_server(
  reader: &mut BufReader<TcpStream>,
  writer: &mut BufWriter<TcpStream>,
  secret: &str,
) -> Result<String, Error> {
  let (_, _) = communication::receive(reader, &["AUTH:"])?;
  communication::send(writer, &format!("AUTH:{secret}\n"))?;
  let (_, _) = communication::receive(reader, &["OK:"])?;

  loop {
    print!("Who? ");
    std::io::stdout().flush().unwrap();
    communication::receive(reader, &["WHO:"])?;
    let input = helpers::read_stdin()?;
    communication::send(writer, &format!("NAME:{}", &input))?;
    let (command, _) = communication::receive(reader, &["NAME_TAKEN:", "OK:"])?;
    match command {
      Commands::OK => {
        return Ok(input);
      }
      Commands::NAMETAKEN => {
        continue;
      }
      _ => {
        return Err(Error::new(ErrorKind::Other, "Negotiations broke down..."));
      }
    }
  }
}
