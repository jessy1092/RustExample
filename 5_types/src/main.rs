#![allow(overflowing_literals)]

use std::mem::size_of_val;

fn main() {
    let decimal = 65.123_f32;

    let integer = decimal as u8;
    let character = integer as char;

    // can not direct casting from decimal to char
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as  u16: {:16b} {}", 1000 as u16, 1000 as u16);
    println!("1000 as   u8: {:16b} {}", 1000 as u8, 1000 as u8);
    println!("  -1 as   u8: {:16b} {}", (-1i8) as u8, (-1i8) as u8);
    println!("1000 mod 256: {:16b} {}", 1000 % 256, 1000 % 256);

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 0?
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1; // default i32
    let f = 1.0; // default f64

    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", size_of_val(&f));

    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem); // Smart inference

    println!("{:?}", vec);

    type NanoSecond = u64;

    let nanosecounds: NanoSecond = 5;

    println!("{}", size_of_val(&nanosecounds));
}
