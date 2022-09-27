use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;

fn byte_line_print(p:u64, byte_vec: [u8; 16]) -> (){
    print!("{:08X} |   ", p);

    for (p, b) in byte_vec.iter().enumerate(){
        if p % 8 == 0 && p != 0{
            print!("| ");
        }
        print!("{:02X} ", b);
    }

    print!("  |");
    for b in byte_vec{
        let b_char = match b as char {
            '\n' => '.',
            '\r' => '.',
            '\0' => '.',
            ' ' => ' ',
            _ => b as char
        };
        print!("{}", b_char);
    }
    print!("|");
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_filename: &str = &args[0];
    let input: File = File::open(input_filename)?;
    
    let mut byte_vec: Vec<[u8; 16]> = Vec::new();

    for (p, b) in input.bytes().enumerate() {
        if p % 16 == 0{
            byte_vec.push([0; 16]);
        }
        let b: u8 = b?;
        byte_vec[p/16][p%16] = b;
    }

    for (p, byte_array) in byte_vec.into_iter().enumerate(){
        byte_line_print(p as u64, byte_array);
    }

    Ok(())
}
