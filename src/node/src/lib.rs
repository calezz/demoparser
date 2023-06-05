#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
use napi::Error;
use parser::parser_settings::rm_user_friendly_names;
use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::variants::OutputSerdeHelperStruct;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[napi]
pub fn parse_chat_messages(file: String) -> Result<Value> {
  let bytes = fs::read(file)?;
  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some("-".to_owned()),
    parse_ents: false,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
  };
  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let mut messages = vec![];
  for i in 0..parser.chat_messages.param1.len() {
    let mut hm: HashMap<String, Option<String>> = HashMap::default();
    hm.insert(
      "entid".to_string(),
      Some(parser.chat_messages.entity_idx[i].unwrap_or(0).to_string()),
    );
    hm.insert(
      "player_name".to_string(),
      parser.chat_messages.param1[i].clone(),
    );
    hm.insert(
      "message".to_string(),
      parser.chat_messages.param2[i].clone(),
    );
    hm.insert(
      "location".to_string(),
      parser.chat_messages.param3[i].clone(),
    );
    hm.insert("param4".to_string(), parser.chat_messages.param4[i].clone());
    messages.push(hm);
  }
  let s = match serde_json::to_value(&messages) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_events(
  path: String,
  event_name: String,
  extra_player: Option<Vec<String>>,
  extra_other: Option<Vec<String>>,
) -> Result<Value> {
  let bytes = fs::read(path)?;

  let player_props = match extra_player {
    Some(p) => p,
    None => vec![],
  };
  let other_props = match extra_other {
    Some(p) => p,
    None => vec![],
  };
  let real_names_player = match rm_user_friendly_names(&player_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let real_names_other = match rm_user_friendly_names(&player_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: real_names_player.clone(),
    wanted_player_props_og_names: player_props,
    wanted_other_props: real_names_other,
    wanted_other_props_og_names: other_props,
    wanted_event: Some(event_name),
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: false,
    count_props: false,
    only_convars: false,
  };
  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let s = match serde_json::to_value(&parser.game_events) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_ticks(path: String, wanted_props: Vec<String>) -> Result<Value> {
  let bytes = fs::read(path)?;
  let real_names = match rm_user_friendly_names(&wanted_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: real_names,
    wanted_player_props_og_names: wanted_props,
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some("".to_string()),
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: false,
    count_props: false,
    only_convars: false,
  };
  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let helper = OutputSerdeHelperStruct {
    inner: parser.output,
  };
  let s = match serde_json::to_value(&helper) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_player_info(path: String) -> Result<Value> {
  let bytes = fs::read(path)?;

  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some("-".to_owned()),
    parse_ents: false,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
  };

  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let mut messages = vec![];
  for i in 0..parser.skins.ent_idx.len() {
    let mut hm: HashMap<String, Option<String>> = HashMap::default();
    let sid = match parser.player_end_data.steamid[i] {
      Some(sid) => Some(sid.to_string()),
      None => None,
    };
    hm.insert("steamid".to_string(), sid);
    let tm = match parser.player_end_data.team_number[i] {
      Some(t) => Some(t.to_string()),
      None => None,
    };
    hm.insert("team_number".to_string(), tm);
    hm.insert("name".to_string(), parser.player_end_data.name[i].clone());
    messages.push(hm)
  }
  let s = match serde_json::to_value(&messages) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
