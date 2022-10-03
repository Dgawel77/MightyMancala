pub fn open_file(str: String) -> std::io::Result<()> {
    println!("open_file is in {}", std::env::current_dir()?.display());
    let result = std::fs::read_to_string(str);
    match result{
        Ok(_) => {println!("Found it in open_file!");}
        Err(_) => {println!("Didnt find it in open_file!");}
    }
    Ok(())
}