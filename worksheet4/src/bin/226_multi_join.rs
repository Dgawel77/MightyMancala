#![feature(slice_concat_trait)]
use std::slice::Join;
use core::borrow::Borrow;

fn join_string_slice() {
    let array: [&str; 4] = ["the", "rain", "in", "Spain"];
    let slice: &[&str] = &array;
    let s: String = slice.join(" ");
    println!("{s}");
}

fn join_vector_slice() {
    let array: [&[i32]; 3] = [&[1, 2], &[3, 4, 5], &[6, 7, 8, 9]];
    let slice: &[&[i32]] = &array;
    let v: Vec<i32> = slice.join::<&[i32]>(&[-2, -1]);
    println!("{v:?}");
}

fn join_multi(){
    // not sure if this is what was meant by the assignment
    let array: &[String] = &[String::from("Hello"), String::from("there"), String::from("!")];
    let vec: Vec<String> = vec![String::from("My"), String::from("name"), String::from("is")];
    let vec_ref: Vec<&str> = vec!["David", "Gawel", "the", "third"];
    let mut s: String = array.join(" ");
    let s1: String = vec.join(" ");
    let s2: String = vec_ref.join(" ");
    s.push_str(&s1);
    s.push_str(&s2);
    println!("{s}");
}

fn main() {
    join_string_slice();
    join_vector_slice();
    join_multi();
}
