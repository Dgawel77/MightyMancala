use std::error::Error;
use std::io::Write;
use std::time::{Duration,SystemTime,SystemTimeError};
use std::str::FromStr;

fn number_of_millis_since_unix_epoch () -> Result<u128, SystemTimeError> {
  let now : SystemTime = SystemTime::now ();
  let duration : Duration = now.duration_since (SystemTime::UNIX_EPOCH)?;
  Ok (duration.as_millis ())
}

fn not_a_random_number (max_value_exclusive : u32) -> Result<u32, SystemTimeError> {
  let millis : u128 = number_of_millis_since_unix_epoch ()?;
  let res : u128 = millis % (max_value_exclusive as u128);
  Ok (res as u32)
}

fn main () -> Result<(), Box<dyn Error>> {
  let randomish : u32 = not_a_random_number (101)?;
  loop {
    println!("Enter a number between 0-100?");
    print!("> ");
    std::io::stdout().flush()?;
    let mut guess_as_string: String = String::with_capacity(64);
    let _num_bytes_read = std::io::stdin().read_line(&mut guess_as_string)?;
    let guess: u32 = u32::from_str(guess_as_string.trim())?;
    if guess == randomish{
        println!("Correct Congratulations!");
        break;
    }else if guess > randomish {
        println!("Your guess is higher.");
    }else if guess < randomish  {
        println!("Your guess is lower.");
    }
  }
  Ok(())
}