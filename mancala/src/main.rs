use clap::Parser;
mod sub;
mod extra_functions;

#[derive(Parser)]
struct Cli{
    name: String,
    path: String,
}

fn main() -> std::io::Result<()>{
    let args = Cli::parse();
    let full_path: String = [args.path, args.name].join("");
    
    println!("{full_path}");
    println!("{}", std::env::current_dir()?.display());

    sub::sub_function();
    extra_functions::extra_function();

    let result = std::fs::read_to_string(full_path);
    match result{
        Ok(_) => {println!("Found it!");}
        Err(_) => {println!("Didnt find it!");}
    }
    Ok(())
}
