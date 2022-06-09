use crate::utils::regex::{check_message, Commands};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

/// Takes a protected reader on a stream, and a set of allowed commands, and attempts to read that from the stream. Errors are propagated upwards.
pub fn receive(
  reader: &mut BufReader<TcpStream>,
  commands: &[&str],
) -> Result<(Commands, String), io::Error> {
  let mut buffer = String::new();
  // let mut locked_reader = reader.lock().unwrap();
  reader.read_line(&mut buffer).unwrap();
  let (command, msg) = check_message(&buffer, commands)?;
  return Ok((command, msg.to_string()));
}

/// Takes a protected writer on a stream, and a string message, and attempts to send that via the stream. Errors are propagated upwards.
pub fn send(writer: &mut BufWriter<TcpStream>, msg: &str) -> Result<(), io::Error> {
  // let mut locked_writer = writer.lock().unwrap();
  writer.write(msg.as_bytes())?;
  writer.flush()?;
  return Ok(());
}
