mod helpers;

use lib::utils::auth::parse_secret;
use lib::utils::regex::Commands;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let args: helpers::ArgsServer = helpers::parse_server("7272");
  let secret = parse_secret(&args.authfile);

  let bind_addr = format!("127.0.0.1:{}", args.port);
  let listener = TcpListener::bind(bind_addr.to_string()).unwrap();

  let mut _clients: Vec<helpers::Client> = Vec::new();

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        let secret = secret.clone();

        thread::spawn(move || {
          println!("starting negotiations");
          handle_client(stream, &secret);
        });
      }
      Err(_e) => {
        println!("Stream failed to init.");
        continue;
      }
    }
  }
}

fn handle_client(stream: TcpStream, secret: &str) {
  let negotiations = helpers::negotiate_with_client(stream, &secret);
  let sleep_time = std::time::Duration::from_secs(3);
  match negotiations {
    Ok(mut client) => loop {
      let incoming =
        helpers::server_receive(&mut client.reader, &["SAY:", "LIST:", "KICK:", "LEAVE:"]);
      match incoming {
        Ok((command, msg)) => match command {
          Commands::SAY => helpers::process_say(command, &msg),
          Commands::LIST => helpers::process_list(command),
          Commands::KICK => helpers::process_kick(command, &msg),
          Commands::LEAVE => helpers::process_leave(command),
          _ => println!("Receieved a message the server shouldnt, {command} - {msg}."),
        },
        Err(err) => {
          println!("Bad Message: {err}");
          thread::sleep(sleep_time);
        }
      }
    },
    Err(err) => {
      println!("Negotiations failed, close stream and end thread: {}", err);
    }
  }
}
