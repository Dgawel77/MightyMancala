use std::io::Write;

fn main() {
    println!("What is your name?");
    print!("> ");
    match std::io::stdout().flush() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error with flush {e:?}");
        }
    }
    let mut name: String = String::with_capacity(64);
    match std::io::stdin().read_line(&mut name) {
        Ok(_num_bytes_read) => {
            let trimmed_name: &str = name.trim();
            println!("Hello {trimmed_name}!");
            println!("Goodbye {trimmed_name}!");
        }
        Err(e) => {
            eprintln!("error with read_line {e:?}");
        }
    }
}
