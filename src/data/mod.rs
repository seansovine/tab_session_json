use chrono::{TimeZone, Utc};
use chrono_tz::US;

use serde::{Deserialize, Deserializer};

use std::cmp;
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

/// Reads in a Tab Session Manager export JSON file, processes
/// and simplifies it, and returns the processed JSON string.
///
/// * `in_file` - An io::Read object for reading text
///               from an input filr or other source.
///
/// If successful, returns a string containing the processed JSON.
///
pub fn process_data<R: io::Read>(in_reader: &mut R) -> Result<String, String> {
  let Ok(mut sessions): Result<Vec<Session>, _> = serde_json::from_reader(in_reader) else {
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

  Ok(string_data)
}
