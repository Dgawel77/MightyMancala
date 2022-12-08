#![allow(dead_code)]

fn string_to_vector(s: &str) -> Vec<u8> {
    s.bytes().collect::<Vec<_>>()
}

fn vector_to_string(v: &[u8]) -> String {
    String::from_utf8_lossy(v).into_owned()
}

fn main() {
    // TODO: try calling dbg!(string_to_vector(...)) and dbg!(vector_to_string(...))
    let new_vec: Vec<u8> = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let multi_vec: Vec<[u8; 3]> = vec![[1,2,3],[1,2,3],[1,2,3],[1,2,3]];
    let string: &str = "hello";
    
    dbg!(string_to_vector(string));
    dbg!(vector_to_string(&new_vec));
    dbg!(vector_to_string(&multi_vec[0]));
    dbg!(multi_vec);
}
