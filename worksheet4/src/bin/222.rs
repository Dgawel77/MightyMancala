// fn f(r: &String) -> &str {
//     &r[2..]
// }

// fn main() {
//     dbg!(f("hello"));
// }
// the arguments are incorrect
// &String expected but &str given

fn f(r: &String) -> &str {
    &r[2..]
}

fn main() {
    dbg!(f(&String::from("hello")));
}
