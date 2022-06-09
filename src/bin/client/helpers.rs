use lib::utils::regex::Commands;
use std::io;
use std::io::Write;

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

pub fn process_enter(_c: Commands, _msg: &str) {}

pub fn process_leave(_c: Commands, _msg: &str) {}

pub fn process_msg(_c: Commands, _msg: &str) {}

pub fn process_kick(_c: Commands) {}

pub fn process_list(_c: Commands, _msg: &str) {}
