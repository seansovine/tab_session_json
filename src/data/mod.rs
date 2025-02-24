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

/// A Serde helper for parsing a millisecond
/// UNIX timestamp tp a formatted datetime string.
fn parse_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let mut timestamp: u64 = u64::deserialize(deserializer).expect("Failed to read date field.");
  timestamp = timestamp / 1000;
  // NOTE: The timestamp in the file is in milliseconds.

  let date_time = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
  let eastern_date_time = date_time.with_timezone(&US::Eastern);

  Ok(eastern_date_time.to_string())
}

/// Read in JSON from file, process and simplify data,
/// and write processed data to file.
///
/// * `in_file` - The path to the Tab Session Manager export JSON,
///               after processing by our Python script.
///
/// * `out_file` - The path at which to write the processed JSON file.
///
pub fn process_data(in_file: &str, out_file: &str) -> Result<(), String> {
  let Ok(file) = fs::File::open(in_file) else {
    return Err(format!("Failed to read file: {}", in_file));
  };
  let reader = io::BufReader::new(file);

  let Ok(mut sessions): Result<Vec<Session>, _> = serde_json::from_reader(reader) else {
    return Err("Failed to parse json.".to_owned());
  };

  // Sort sessions by date.
  sessions.sort_by_key(|entry| cmp::Reverse(entry.date.clone()));
  // Sort tabs in session by original index.
  sessions.iter_mut().for_each(|session| {
    session.windows.iter_mut().for_each(|window| {
      window.tabs.sort_by_key(|tab| tab.index);
    });
  });
  let export = Export { sessions };

  let Ok(string_data) = serde_json::to_string_pretty(&export) else {
    return Err("Failed to serialize processed data.".to_owned());
  };

  fs::write(out_file, string_data).expect("Failed to write processed data to file.");

  Ok(())
}
