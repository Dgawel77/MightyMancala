#![feature(type_ascription)]

fn main () {
  let v : Vec<i32> = Vec::from_iter (0..=10);

  // Specify Vec<_> on the variable.
  let res : Vec<_> = v.iter ().map (|&x| x * x).filter (|&x| x > 5).collect ();
  println! ("{res:?}");

  // Specify Vec<_> using the turbofish syntax to pass a type argument to the collect method.
  println! ("{:?}", v.iter ().map (|&x| x * x).filter (|&x| x > 5).collect::<Vec<_>> ());

  // Specify Vec<_> using type ascription.
  println! ("{:?}", v.iter ().map (|&x| x * x).filter (|&x| x > 5).collect () : Vec<_>);
}