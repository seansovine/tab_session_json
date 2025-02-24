use chrono::{TimeZone, Utc};
use chrono_tz::US;

use serde::{Deserialize, Deserializer};

use std::cmp;
use std::fs;
use std::io;

use serde_json;

//////////////////////////////
// Data struct definitions. //
//////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct Export {
  sessions: Vec<Session>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Session {
  name: String,
  // We parse unix timestamp to date + time.
  #[serde(deserialize_with = "parse_date")]
  date: String,
  windows: Vec<Window>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Window {
  tabs: Vec<Tab>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tab {
  title: String,
  url: String,
  // We read this in to sort according to extension's order.
  #[serde(skip_serializing)]
  index: u64,
}

//////////////////////////////////////////
// Public and private parsing functions //
//////////////////////////////////////////

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

pub fn process_data(in_file: &str, out_file: &str) -> () {
  let file = fs::File::open(in_file).expect(&format!("Failed to read file: {}", in_file));
  let reader = io::BufReader::new(file);

  let mut sessions: Vec<Session> =
    serde_json::from_reader(reader).expect("Failed to parse json with serde.");

  // Sort sessions by date.
  sessions.sort_by_key(|entry| cmp::Reverse(entry.date.clone()));
  // Sort tabs in session by original index.
  sessions.iter_mut().for_each(|session| {
    session.windows.iter_mut().for_each(|window| {
      window.tabs.sort_by_key(|tab| tab.index);
    });
  });

  let export = Export { sessions };
  let string_data = serde_json::to_string_pretty(&export)
    .expect("Failed to serialize processed data using serde_json.");

  fs::write(out_file, string_data).expect("Failed to write processed data to file.");
}
