use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_filename: &str = &args[0];
    let data: String = std::fs::read_to_string(input_filename)?;
    //return counts as 2 charachters newline and carrige return
    println!("{}", data.len());
    for (p, c) in data.chars().enumerate().into_iter(){
        if p % 8 == 0{
            println!();
        }
        print!("{c}");
    }
    println!();
    println!("{}", data);
    Ok(())
}
