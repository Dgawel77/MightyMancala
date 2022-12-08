fn reverse_string (s : String) -> String {
    let mut rev_vec: Vec<u8> = Vec::new();
    for c in s.chars().rev().into_iter(){
        rev_vec.push(c as u8);
    }
    let ret_string: String = String::from_utf8(rev_vec).expect("Found invalid UTF-8");
    ret_string
}

fn main () {
    let s : String = String::from ("Hello, world!");
    println! ("{}", reverse_string (s));
}
