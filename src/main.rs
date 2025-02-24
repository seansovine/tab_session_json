////////////////////////////////////////////////////////////////
/// Program to read the Tab Session Manager JSON export file. //
///                                                           //
/// Created by Sean Sovine on 2025-02-23.                     //
////////////////////////////////////////////////////////////////;

#[macro_use]
extern crate serde_derive;

mod data;

use clap::Parser;

#[derive(Parser)]
struct Args {
  #[clap(long)]
  in_file: String,
  #[clap(long)]
  out_file: String,
}

fn main() -> Result<(), String> {
  let args = Args::parse();
  data::process_data(&args.in_file, &args.out_file)?;

  Ok(())
}
