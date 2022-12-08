fn f(r: &str) -> &str {
    &r[2..]
}

fn main() {
    //this is a &str so no wonder it works
    dbg!(f("hello"));
    //this is a &String type but rust implicity converts it to 
    //a &str by removing one layer of indirection
    dbg!(f(&String::from("hello")));
}
