use lib::utils::regex::{check_message, ClientCommands};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct ArgsClient {
  pub name: String,
  pub authfile: String,
  pub port: u32,
}

pub fn parse_client() -> ArgsClient {
  let name: String = std::env::args().nth(1).expect("no name given");
  let authfile: String = std::env::args().nth(2).expect("no authfile given");
  let possible_port: String = std::env::args().nth(3).expect("no port given");
  let port: u32 = possible_port
    .parse::<u32>()
    .ok()
    .expect(&format!("port must be a u32, received {}", possible_port));
  return ArgsClient {
    name,
    authfile,
    port,
  };
}

/// Reads standard input for user typed input, ending at a newline. Any errors are propagated upwards.
pub fn read_stdin() -> Result<String, io::Error> {
  let mut stdout = io::stdout();
  let stdin = io::stdin();
  let mut buffer = String::new();
  stdin.read_line(&mut buffer)?;
  stdout.flush()?;
  return Ok(buffer.to_string());
}

/// Takes a protected reader on a stream, and a set of allowed commands, and attempts to read that from the stream. Errors are propagated upwards.
pub fn receive(
  reader: &mut Arc<Mutex<BufReader<TcpStream>>>,
  commands: &[&str],
) -> Result<(ClientCommands, String), io::Error> {
  println!("starting receive");
  let mut buffer = String::new();
  // let r = Arc::clone(&reader);
  let mut locked_reader = reader.lock().unwrap();
  println!("Got lock on reader");
  locked_reader.read_line(&mut buffer).unwrap();
  println!("Got msg: {}", buffer);
  let res = check_message(&buffer, commands);
  println!("Checked message");

  match res {
    Ok((command, msg)) => {
      return Ok((command, msg.to_string()));
    }
    Err(_err) => {
      return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
    }
  }
}

/// Takes a protected writer on a stream, and a string message, and attempts to send that via the stream. Errors are propagated upwards.
pub fn send(writer: &mut Arc<Mutex<BufWriter<TcpStream>>>, msg: &str) -> Result<(), io::Error> {
  let mut locked_writer = writer.lock().unwrap();
  locked_writer.write(msg.as_bytes())?;
  locked_writer.flush()?;
  return Ok(());
}
