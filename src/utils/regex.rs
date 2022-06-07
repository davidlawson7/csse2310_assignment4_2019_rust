use std::str::FromStr;

/// Checks a value for one of the patterns provided. Upon finding that pattern, returns whether it found it, followed
/// by two string slices. The first being the pattern, and the second being the remaining string.
/// If no pattern is found, false is returned along with the entire string for the two slices.
pub fn check_message<'text>(
  text: &'text str,
  patterns: &[&str],
) -> Result<(ClientCommands, &'text str), &'static str> {
  println!("message {}", text);
  let opt = match_str_patterns(text, patterns).ok();

  if !opt.is_none() {
    let res = opt.unwrap();
    return Ok(res);
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
  AUTH,
  SAY,
  LIST,
  KICK,
  LEAVE,
  NAME,
  NAMETAKEN,
  WHO,
  OK,
}

impl FromStr for ClientCommands {
  type Err = ();

  fn from_str(input: &str) -> Result<ClientCommands, Self::Err> {
    match input {
      "SAY" | "SAY:" => Ok(ClientCommands::SAY),
      "LIST" | "LIST:" => Ok(ClientCommands::LIST),
      "KICK" | "KICK:" => Ok(ClientCommands::KICK),
      "LEAVE" | "LEAVE:" => Ok(ClientCommands::LEAVE),
      "AUTH" | "AUTH:" => Ok(ClientCommands::AUTH),
      "NAME" | "NAME:" => Ok(ClientCommands::NAME),
      "WHO" | "WHO:" => Ok(ClientCommands::WHO),
      "NAMETAKEN" | "NAMETAKEN:" => Ok(ClientCommands::NAMETAKEN),
      "OK" | "OK:" => Ok(ClientCommands::OK),
      _ => Err(()),
    }
  }
}
