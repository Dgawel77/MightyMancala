use std::env;
use std::error::Error;

fn reverse_string (s : String) -> String {
    let mut rev_vec: Vec<u8> = Vec::new();
    for c in s.chars().rev().into_iter(){
        rev_vec.push(c as u8);
    }
    let ret_string: String = String::from_utf8(rev_vec).expect("Found invalid UTF-8");
    ret_string
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_filename: &str = &args[0];
    let data: String = std::fs::read_to_string(input_filename)?;
    //return counts as 2 charachters newline and carrige return
    println!("{}", reverse_string(data));
    Ok(())
}