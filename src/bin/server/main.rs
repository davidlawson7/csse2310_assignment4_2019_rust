mod handlers;
mod helpers;
mod structures;

use self::handlers::handle_client;
use self::helpers::{parse_server, ArgsServer};
use self::structures::Client;
use lib::utils::auth::parse_secret;

use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
  let args: ArgsServer = parse_server("7272");
  let secret = parse_secret(&args.authfile);
  println!("Secret is {}", secret);

  let bind_addr = format!("127.0.0.1:{}", args.port);
  let listener = TcpListener::bind(bind_addr.to_string()).unwrap();

  let mut handles: Vec<JoinHandle<()>> = vec![];

  let clients: HashMap<String, Client> = HashMap::new();

  let protected_clients = Arc::new(Mutex::new(clients));

  println!("Server starting on {}", bind_addr);

  // accept connections and process them serially, ignore bad connections
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        let protected_clients = Arc::clone(&protected_clients);
        let handle = thread::spawn(move || {
          handle_client(stream, protected_clients).unwrap();
          println!("hihihih")
        });
        handles.push(handle);
      }
      Err(_e) => continue,
    }
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
