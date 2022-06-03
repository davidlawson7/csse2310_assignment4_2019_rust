// SAY:
// LIST:
// KICK:
// LEAVE:

use std::str::FromStr;

/// Checks a value for one of the patterns provided. Upon finding that pattern, returns whether it found it, followed
/// by two string slices. The first being the pattern, and the second being the remaining string.
/// If no pattern is found, false is returned along with the entire string for the two slices.
pub fn check_message<'text>(
  text: &'text str,
  patterns: &[&str],
) -> Result<(ClientCommands, &'text str), &'static str> {
  let opt = match_str_patterns(text, patterns).ok();

  if !opt.is_none() {
    let (pattern, contents) = opt.unwrap();
    match pattern {
      ClientCommands::SAY | ClientCommands::LIST | ClientCommands::KICK | ClientCommands::LEAVE => {
        return Ok((pattern, contents));
      }
    }
  }
  return Err("Bad message");
}

fn match_str_patterns<'text>(
  text: &'text str,
  patterns: &[&str],
) -> Result<(ClientCommands, &'text str), &'static str> {
  for &pattern in patterns.iter() {
    let check = match_str(text, pattern).ok();
    let pattern_enum = ClientCommands::from_str(&pattern).ok();
    if !check.is_none() && !pattern_enum.is_none() {
      return Ok((pattern_enum.unwrap(), check.unwrap()));
    }
  }
  return Err("message formatted badly");
}

fn match_str<'text>(text: &'text str, pattern: &str) -> Result<&'text str, &'static str> {
  let pattern_len = pattern.chars().count();

  for (i, _value) in text.chars().enumerate() {
    let curr = &text[0..i];
    let curr_len = curr.chars().count();

    if curr_len > pattern_len {
      return Err("Text now to long for current pattern");
    }

    if curr_len != pattern_len {
      continue;
    }

    if curr.eq(pattern) {
      return Ok(&text[i..]);
    }
  }

  return Err("Text is to short to contain pattern, or is not equel");
}

pub enum ClientCommands {
  SAY,
  LIST,
  KICK,
  LEAVE,
}

impl FromStr for ClientCommands {
  type Err = ();

  fn from_str(input: &str) -> Result<ClientCommands, Self::Err> {
    match input {
      "SAY" => Ok(ClientCommands::SAY),
      "SAY:" => Ok(ClientCommands::SAY),
      "LIST" => Ok(ClientCommands::LIST),
      "LIST:" => Ok(ClientCommands::LIST),
      "KICK" => Ok(ClientCommands::KICK),
      "KICK:" => Ok(ClientCommands::KICK),
      "LEAVE" => Ok(ClientCommands::LEAVE),
      "LEAVE:" => Ok(ClientCommands::LEAVE),
      _ => Err(()),
    }
  }
}
