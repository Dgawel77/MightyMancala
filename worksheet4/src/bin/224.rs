// expects a life time parameter
// fn f1() -> &str {
//     "hey"
// }

// still expects a life time parameter
// fn f2 (s : String) -> &str {
//     s.as_str()
// }

// this works fine
fn f3(r: &String) -> &str {
    r
}

// so does this
fn f4(r: &str) -> &str {
    r
}

fn main() {
    // f1();
    // f2();
    dbg!(f3(&String::from("hello")));
    dbg!(f4("hello"));
}
