fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    let mut w: () = v.iter();
    while let Some (n) = w.next () {
        let n: () = n;
        println!("{n}");
    }
}
