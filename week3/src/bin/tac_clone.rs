use std::env;
use std::error::Error;

fn main () -> Result<(), Box<dyn Error>> {
  let args : Vec<String> = env::args ().skip (1).collect ();
  let input_filename : &str = &args[0];
  let data : String = std::fs::read_to_string (input_filename)?;
  
  let mut out_vec: Vec<String> = Vec::new();
  for line in data.lines () {
    out_vec.push(format!("{}", line));
  }
  for line in out_vec.iter().rev(){
    println!("{}", line);
  }
  Ok (())
}