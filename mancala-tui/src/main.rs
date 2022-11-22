mod game;
mod mancala;
mod board;
mod tests;

fn main(){
    game::run();
}

pub mod lib{
    use std::str;
    use std::fs::{File, read};
    use std::io::{self, BufRead, Read};
    use std::path::Path;

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn read_string<P>(filename: P) -> String
    where P: AsRef<Path>, {
        let mut file = File::open(filename).expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Error while reading file");
        data
    }
}