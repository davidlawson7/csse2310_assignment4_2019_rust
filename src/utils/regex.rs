use std::io;
use std::str::FromStr;

static STRING_TO_LONG: &str = "STRING_TO_LONG";
static STRING_TO_SHORT: &str = "STRING_TO_SHORT";
static STRING_NOT_MATCH: &str = "STRING_NOT_MATCH";
static INVALID_COMMAND: &str = "INVALID_COMMAND";

/// Checks a value for one of the patterns provided. Upon finding that pattern, returns whether it found it, followed
/// by two string slices. The first being the pattern, and the second being the remaining string.
/// If no pattern is found, false is returned along with the entire string for the two slices.
pub fn check_message(text: &str, patterns: &[&str]) -> Result<(Commands, String), io::Error> {
  for &pattern in patterns.iter() {
    match match_str(text, pattern) {
      Ok(msg) => {
        let command = Commands::from_str(pattern).expect("Provided pattern must match a command enum. Check hardcoded enums in receiver function calls.");
        let mut trimmed_msg = msg.to_string();
        trim_newline(&mut trimmed_msg);
        return Ok((command, trimmed_msg));
      }
      Err(_msg) => {
        continue;
      }
    }
  }
  return Err(io::Error::new(io::ErrorKind::Unsupported, INVALID_COMMAND));
}

pub fn match_str<'text>(text: &'text str, pattern: &str) -> Result<&'text str, &'static str> {
  let pattern_len = pattern.chars().count();
  if text.chars().count() < pattern_len {
    return Err(STRING_TO_SHORT);
  }

  for (i, _value) in text.chars().enumerate() {
    let curr = &text[0..i];
    let curr_len = curr.chars().count();

    if curr_len > pattern_len {
      return Err(STRING_TO_LONG);
    }

    if curr_len != pattern_len {
      continue;
    }

    if curr.eq(pattern) {
      return Ok(&text[i..]);
    }
  }

  return Err(STRING_NOT_MATCH);
}

pub enum Commands {
  AUTH,
  WHO,
  NAMETAKEN,
  OK,
  ENTER,
  LEAVE,
  MSG,
  KICK,
  LIST,
  NAME,
  SAY,
}

impl FromStr for Commands {
  type Err = ();

  fn from_str(input: &str) -> Result<Commands, Self::Err> {
    match input {
      "LIST" | "LIST:" => Ok(Commands::LIST),
      "KICK" | "KICK:" => Ok(Commands::KICK),
      "LEAVE" | "LEAVE:" => Ok(Commands::LEAVE),
      "AUTH" | "AUTH:" => Ok(Commands::AUTH),
      "NAME" | "NAME:" => Ok(Commands::NAME),
      "WHO" | "WHO:" => Ok(Commands::WHO),
      "NAME_TAKEN" | "NAME_TAKEN:" => Ok(Commands::NAMETAKEN),
      "OK" | "OK:" => Ok(Commands::OK),
      "SAY" | "SAY:" => Ok(Commands::SAY),
      _ => Err(()),
    }
  }
}

impl std::fmt::Display for Commands {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Commands::LIST => return write!(f, "{}", "LIST"),
      Commands::KICK => return write!(f, "{}", "KICK"),
      Commands::LEAVE => return write!(f, "{}", "LEAVE"),
      Commands::AUTH => return write!(f, "{}", "AUTH"),
      Commands::NAME => return write!(f, "{}", "NAME"),
      Commands::WHO => return write!(f, "{}", "WHO"),
      Commands::NAMETAKEN => return write!(f, "{}", "NAME_TAKEN"),
      Commands::OK => return write!(f, "{}", "OK"),
      Commands::ENTER => return write!(f, "{}", "ENTER"),
      Commands::MSG => return write!(f, "{}", "MSG"),
      Commands::SAY => return write!(f, "{}", "SAY"),
    };
  }
}

fn trim_newline(s: &mut String) {
  if s.ends_with('\n') {
    s.pop();
    if s.ends_with('\r') {
      s.pop();
    }
  }
}
