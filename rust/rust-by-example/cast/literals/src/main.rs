use std::mem::*;

fn main() {

    let x = 1_u8;
    let y = 2_u32;
    let z = 3_f32;

    let a = 0_u64;

    let i = 1;
    let f = 1.0;

    println!("size if 'x' in bytes: {}", size_of_val(&x));
    println!("size if 'y' in bytes: {}", size_of_val(&y));
    println!("size if 'z' in bytes: {}", size_of_val(&z));

    println!("size if 'a' in bytes: {}", size_of_val(&a));

    println!("size if 'i' in bytes: {}", size_of_val(&i));
    println!("size if 'f' in bytes: {}", size_of_val(&f));

}
