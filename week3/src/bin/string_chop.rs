fn chop_1 (s : String) -> Vec<char> {
    let mut return_vec: Vec<char> = Vec::new();
    for c in s.chars(){
        return_vec.push(c);
    }
    return_vec
}

fn main () {
    let s : String = String::from ("Hello, world!");
    println! ("{:?}", chop_1 (s.clone ()));

    let s1 : String = String::from ("What is the time in Belgium?");
    println! ("{:?}", chop_1 (s1.clone ()));
    
    let s2 : String = String::from ("Hola Señor");
    println! ("{:?}", chop_1 (s2.clone ()));
}
