fn examine_i32() {
    println!("examine_i32");

    let x: i32 = 0x11223344;

    // print the address of x, shorter version
    println!("{:016p}", &x);

    // print the address of x, longer version
    let r: &i32 = &x;
    println!("{:016p}", r);

    // turn reference of type "&i32" into pointer of type "*const i32"
    let p: *const i32 = r;
    println!("{:016p}", p);

    // read 4 memory locations pointed to by p as an i32, and print the result out
    println!("{:08x}", unsafe { *p });

    // same as above but without the use of other statements
    println!("{:08x}", {
        let p: *const i32 = &x;
        unsafe { *p }
    });

    // print 4 memory locations pointed to by p individually
    for i in 0..4 {
        let p: *const i32 = &x;
        // it is unsafe to convert a pointer to i32 into a pointer to u8, so force it
        let q: *const u8 = unsafe { std::mem::transmute(p) };
        // arithmetic is not defined on Rust pointers, so we have to cast to/from usize first
        let q = ((q as usize) + i) as *const u8;
        println!("{:02x}", unsafe { *q });
    }

    println!();
    // same as above but simplifie and with variable pulled out of loop
    let mut q: *const u8 = unsafe { std::mem::transmute(&x) };
    for _ in 0..4 {
        println!("{:02x}", unsafe { *q });
        q = ((q as usize) + 1) as *const u8;
    }
}

fn examine_i64(){
    println!("examine_i64");

    let x: i64 = 0x1122334455667788;

    // print the address of x, shorter version
    println!("{:016p}", &x);

    // print the address of x, longer version
    let r: &i64 = &x;
    println!("{:016p}", r);

    // turn reference of type "&i64" into pointer of type "*const i64"
    let p: *const i64 = r;
    println!("{:016p}", p);

    // read 8 memory locations pointed to by p as an i64, and print the result out
    println!("{:016x}", unsafe { *p });

    // same as above but without the use of other statements
    println!("{:016x}", {
        let p: *const i64 = &x;
        unsafe { *p }
    });

    // print 8 memory locations pointed to by p individually
    for i in 0..8 {
        let p: *const i64 = &x;
        // it is unsafe to convert a pointer to i32 into a pointer to u8, so force it
        let q: *const u8 = unsafe { std::mem::transmute(p) };
        // arithmetic is not defined on Rust pointers, so we have to cast to/from usize first
        let q = ((q as usize) + i) as *const u8;
        println!("{:02x}", unsafe { *q });
    }

    println!();
    // same as above but simplified and with variable pulled out of loop
    let mut q: *const u8 = unsafe { std::mem::transmute(&x) };
    for _ in 0..8 {
        println!("{:02x}", unsafe { *q });
        q = ((q as usize) + 1) as *const u8;
    }
}

fn main() {
    examine_i32();
    println!();
    examine_i64();
}
