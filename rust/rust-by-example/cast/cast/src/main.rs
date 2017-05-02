// #![allow(overflowing_literals)]

fn main() {

    let decimal = 65.4321_f32;

    let integer: u8 = decimal as u8;
    let character = integer as char;

    println!("Casting {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is {}", 1000 as u16);
    println!("100000 as a u16 is {}", 100000 as u16); // 100000 - 65536
    println!("1000 as a u8 is {}", 1000 as u8); // 1000 - 256 * 3
    println!("-1_i8 as a u8 is {}", -1_i8 as u8); // -1 + 256
    println!("-1_i8 as a u32 is {}", -1_i8 as u16); // -1 + 65536

    println!(" 128 as a i8 is : {}", 128_u16 as i8);
}
