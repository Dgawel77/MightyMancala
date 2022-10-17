fn main() {
    
    let mut s : String = String::from("hello");
    s.push('!');
    dbg!(s);

    let mut s2 : String = String::from("hello");
    s2.push_str(" what is the time?");
    dbg!(s2);

    let mut v : Vec<&str> = vec!["hello"];
    v.push("there!");
    dbg!(v);

    let mut v2 : Vec<&str> = vec!["hello"];
    v2.extend(vec!["my", "name", "is", "david"]);
    dbg!(v2);

}
