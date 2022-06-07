use crate::structures::Client;
use lib::utils::regex::ClientCommands;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

/// Handles a new client connecting to the chat server.
pub fn handle_client(
  stream: TcpStream,
  _clients: Arc<Mutex<HashMap<String, Client>>>,
) -> Result<&'static str, &'static str> {
  // Initialise the client
  let mut client = negotiate_with_client(stream)?;

  loop {
    let incoming = client.receive(&["SAY:", "LIST:", "KICK:", "LEAVE:"]);
    match incoming {
      Ok((command, _msg)) => {
        println!("Client Said: {}", _msg);
        match command {
          ClientCommands::LEAVE => {
            return Ok("Graceful Leaving");
          }
          _ => todo!(),
        }
      }
      Err(err) => {
        if err.eq("bad") {
          return Err("bad");
        }
      }
    }
  }
}

fn negotiate_with_client(stream: TcpStream) -> Result<Client, &'static str> {
  let mut client = Client::new(stream);

  // Do the auth challenge
  println!("sending challange");
  client.send("AUTH:");
  let (_, secret) = client.receive(&["AUTH:"])?;
  println!("Received: {}", secret);
  client.send("OK:");

  // Negotiate name
  loop {
    client.send("WHO:");
    let (_, proposed_name) = client.receive(&["NAME:"])?;
    if proposed_name.chars().count() == 0 || proposed_name == "Fred" {
      client.send("NAME_TAKEN:");
      continue;
    }
    client.set_name(proposed_name);
    break;
  }
  client.send("OK:");
  return Ok(client);
}
