use std::fmt::Error;

fn main() {
    dbg!(std::mem::size_of::<Box<String>>());
    // add more copies of the above line (using different types)
    dbg!(std::mem::size_of::<u32>());
    //4 correct
    dbg!(std::mem::size_of::<()>());
    //0 correct
    dbg!(std::mem::size_of::<u128>());
    //8 wrong 16
    dbg!(std::mem::size_of::<i128>());
    //8 wrong 16
    dbg!(std::mem::size_of::<String>());
    //3 correct 24 bits - 3 bytes
    dbg!(std::mem::size_of::<&String>());
    //2 wrong 8 - 1 byte
    dbg!(std::mem::size_of::<&str>());
    //2 correct kind of 16 bits - 2 bytes
    dbg!(std::mem::size_of::<Result<u32, Error>>());
    //8? suprisingly correct 8 bits more than a u32
    dbg!(std::mem::size_of::<Option<u32>>());
    //8? suprisingly correct 8 bits more than a u32 but the same as the result
}
