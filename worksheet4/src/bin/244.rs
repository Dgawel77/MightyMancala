// print_type_of function and print_type macro from https://notes.iveselov.info/programming/refs-and-pattern-matching-in-rust

fn print_type_of<T>(msg: &str, _: T) {
    println!("type of {}: {}", msg, std::any::type_name::<T>());
}

macro_rules! print_type {
    ( $x:expr ) => {
        print_type_of(stringify!($x), &$x)
    };
}

fn main() {
    let v: Vec<i32> = Vec::from_iter(0..=10);
    let x: &str = "Hello";
    let y: &String = &String::from("David");
    let z: String = String::from("Gawel");
    print_type!(x);
    print_type!(y);
    print_type!(z);
    print_type!(v);
    print_type!(v.iter().map(|&x| x * x).filter(|&x| x > 5));
}
