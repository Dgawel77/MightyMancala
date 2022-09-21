fn increment_array (mut arr : [u8; 10]) -> [u8; 10] {
    // TODO: add one to every element of the array "arr", then return the array.
    for p in 0..arr.len() {
        arr[p] += 1;
    }
    return arr;
  }
  
  fn increment_array_ref (arr_ref : &mut [u8; 10]) {
    // TODO: add one to every element of the array "arr".  Nothing to return.
    for p in 0..(*arr_ref).len() {
        (*arr_ref)[p] += 1;
    }
  }
  
  fn increment_slice (slice : &mut [u8]) {
    // TODO: add one to every element of the array "arr".  Nothing to return.
    for p in 0..(*slice).len() {
        (*slice)[p] += 1;
    }
  }
  
  fn main () {
    let mut array : [u8; 10] = [4,5,6,7,8,9,5,5,6,10];
    dbg! (array);
    dbg! (increment_array (array));
    dbg! (array);
    dbg! (increment_array_ref (&mut array));
    dbg! (array);
    dbg! (increment_slice (&mut array));
    dbg! (array);
  }