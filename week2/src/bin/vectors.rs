fn main(){
    let mut macro_vec : Vec<u8> = vec![1,2,3];
    dbg!( macro_vec.len(), macro_vec.capacity(), macro_vec.as_ptr());
    let mut new_vec : Vec<u8> = Vec::new();
    dbg!( new_vec.len(), new_vec.capacity(), new_vec.as_ptr());
    let mut capacity_vec : Vec<u8> = Vec::with_capacity(10);
    dbg!( capacity_vec.len(), capacity_vec.capacity(), capacity_vec.as_ptr());
    
    for x in 0..10{
        macro_vec.push(x as u8);
    }
    dbg!( macro_vec.len(), macro_vec.capacity(), macro_vec);

    for x in 0..10{
        new_vec.push(x as u8);
    }
    dbg!( new_vec.len(), new_vec.capacity(), new_vec.as_ptr(), new_vec);

    for x in 0..10{
        capacity_vec.push(x as u8);
    }
    dbg!( capacity_vec.len(), capacity_vec.capacity(), capacity_vec);
}