fn chop_3 (s : String) -> Vec<[char; 3]> {
    let mut ret_vec: Vec<[char; 3]> = Vec::new();
    for (p, c) in s.chars().enumerate(){
        if p % 3 == 0{
            let addition = ['_','_','_'];
            ret_vec.push(addition);
        }
        ret_vec[p/3][p%3] = c;
    }
    ret_vec
}

fn main () {
    let s : String = String::from ("Hello, world ");
    println! ("{:?}", chop_3(s.clone ()));
}
