////////////////////////////////////////////////////////////////
/// Program to read the Tab Session Manager JSON export file. //
///                                                           //
/// Created by Sean Sovine on 2025-02-23.                     //
////////////////////////////////////////////////////////////////

#[macro_use]
extern crate serde_derive;

use chrono::{TimeZone, Utc};
use chrono_tz;

use chrono_tz::US;
use serde::{Deserialize, Deserializer};

use std::fs;
use std::io;

use serde_json;

const FILE: &str = "/home/sean/Backup/362 Sessions - 2025-02-23 15-47-40.json";

fn main() {
  let file = fs::File::open(FILE).expect(&format!("Failed to read file: {}", FILE));
  let reader = io::BufReader::new(file);

  let mut sessions: Vec<Session> =
    serde_json::from_reader(reader).expect("Failed to parse json with serde.");
  sessions.sort_by_key(|entry| entry.date.clone());

  println!("Found data:\n\n{:#?}", sessions);
}

/////////////////////////////
// Data struct definition. //
/////////////////////////////

#[derive(Deserialize, Debug)]
struct Session {
  #[serde(rename = "name")]
  _name: String,

  // We parse unix timestamp to date + time.
  #[serde(deserialize_with = "parse_date")]
  date: String,

  #[serde(rename = "windows")]
  _windows: Vec<Window>,
}

#[derive(Deserialize, Debug)]
struct Window {
  // Bummer, the JSON here is not formatted well for our purposes:
  // It uses sequential integer keys where we would rather have a list
  // entry. Maybe we can first write a quick Python script to convert
  // the data, before calling this program to parse it.
}

fn parse_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let mut timestamp: u64 = u64::deserialize(deserializer).expect("Failed to read date field.");
  timestamp = timestamp / 1000;

  // NOTE: The timestamp in the file is in milliseconds.
  let date_time = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
  let eastern_date_time = date_time.with_timezone(&US::Eastern);

  Ok(String::from(eastern_date_time.to_string()))
}
