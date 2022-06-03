use std::fs;

pub fn parse_secret(filename: &str) -> String {
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let bytes = contents.as_bytes();

  for (i, &byte) in bytes.iter().enumerate() {
    if byte == b'\n' {
      return contents[0..i].to_owned();
    }
  }
  return contents[..].to_owned();
}
