use std::error::Error;
use std::io::Write;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    // ask for the name
    println!("What is your name?");
    print!("> ");
    std::io::stdout().flush()?;
    let mut name: String = String::with_capacity(64);
    let _num_bytes_read = std::io::stdin().read_line(&mut name)?;
    let trimmed_name: &str = name.trim();
    println!("Hello {trimmed_name}!");
    
    // ask for the age
    println!("What is your age?");
    print!("> ");
    std::io::stdout().flush()?;
    let mut age_as_str: String = String::with_capacity(64);
    let _num_bytes_read = std::io::stdin().read_line(&mut age_as_str)?;
    let age: u32 = u32::from_str(age_as_str.trim())?;
    println!("You are {age} years old!");
    println!("Goodbye {trimmed_name}!");
    
    Ok(())
}
