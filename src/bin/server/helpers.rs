pub struct ArgsServer {
  pub authfile: String,
  pub port: u32,
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
