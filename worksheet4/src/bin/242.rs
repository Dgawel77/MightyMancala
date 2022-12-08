fn main () {
    let v : Vec<i32> = Vec::from_iter (0..=10);
    let tmp0: std::slice::Iter<'_, i32> = v.iter ();
    let tmp1 = tmp0.map (|&x| x * x);
    let tmp2 = tmp1.filter (|&x| x > 5);
    let res: Vec<i32> = tmp2.collect::<Vec<_>> ();
    println! ("{res:?}");
  }
