use std::env;
use std::error::Error;

fn main () -> Result<(), Box<dyn Error>> {
  let args : Vec<String> = env::args ().skip (1).collect ();
  let input_filename : &str = &args[0];
  let data : String = std::fs::read_to_string (input_filename)?;
  for line in data.lines () {
    println!("{}\t>>>{}<<<",line.len(), line);
  }
  Ok (())
}