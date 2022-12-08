fn f(s: String) -> String {
    let mut s2: String = String::from(s);
    s2.push_str("world");
    s2
}

fn g(s: String) -> &'static str {
    //does not seem correct
    static s1: &'static str = "helloworld";
    s1
}

fn main() {
    let orig = String::from("hello");
    dbg!(f(orig.clone()));
    dbg!(g(orig.clone()));
}
