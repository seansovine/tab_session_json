////////////////////////////////////////////////////////////////
/// Program to read the Tab Session Manager JSON export file. //
///                                                           //
/// Created by Sean Sovine on 2025-02-23.                     //
////////////////////////////////////////////////////////////////;

#[macro_use]
extern crate serde_derive;

mod data;

use clap::Parser;

use std::fs;
use std::io;

#[derive(Parser)]
struct Args {
  #[clap(long)]
  in_file: String,
  #[clap(long)]
  out_file: String,
}

fn main() -> Result<(), String> {
  let args = Args::parse();

  let Ok(file) = fs::File::open(&args.in_file) else {
    return Err(format!("Failed to read file: {}", &args.in_file));
  };
  let mut reader = io::BufReader::new(file);

  let out_string = data::process_data(&mut reader)?;
  fs::write(&args.out_file, out_string).expect("Failed to write processed data to file.");

  Ok(())
}
