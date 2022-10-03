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
    
    println!("full path is {full_path}");
    println!("main is in {}", std::env::current_dir()?.display());

    sub::sub_function();
    extra_functions::open_file(full_path.clone()).expect("it aint work");

    let result = std::fs::read_to_string(full_path);
    match result{
        Ok(_) => {println!("Found it in main.rs!");}
        Err(_) => {println!("Didnt find it in main.rs!!");}
    }

    let set_file_path = "C:/Users/David Gawel/OneDrive/Programs/CSC362/csc363-20223-gaweldd7/week3/src/bin/hello.txt";
    let relative_file_path = "../../week3/src/bin/hello.txt";

    let result = std::fs::read_to_string(set_file_path);
    match result{
        Ok(_) => {println!("Found it in main.rs with the set path!");}
        Err(_) => {println!("Didnt find it in main.rs with the set path!!");}
    }

    let result = std::fs::read_to_string(relative_file_path);
    match result{
        Ok(_) => {println!("Found it in main.rs with the relative path!");}
        Err(_) => {println!("Didnt find it in main.rs with the relative path!");}
    }

    Ok(())
}
