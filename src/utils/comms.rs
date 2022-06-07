pub fn send(msg: &str, reader: &mut BufReader<TcpStream>) {
  writer.write(msg.as_bytes()).unwrap();
  writer.flush().unwrap();
}

pub fn receive() {
  let mut buffer = String::new();
  let incoming = reader.read_line(&mut resp_buffer);
  match incoming {
    Ok(_size) => {
      let res_from_client = check_message(&resp_buffer, &["AUTH:"]).ok();
      let (_command, secret) = res_from_client.unwrap();
      println!("Secret from client: {}", secret);
    }
    Err(_err) => {
      println!("Failed to negotiate with the client");
    }
  }
}
