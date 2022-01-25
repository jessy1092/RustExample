use std::mem::size_of;
use std::mem::size_of_val;

fn main() {
    let x: bool = true;

    println!("Bool: {} size_of: {}", x, size_of::<bool>());

    let x: char = 'a';

    println!("Char: {} size_of: {}", x, size_of::<char>());
    println!("Char: {} size_of_val: {}", x, size_of_val(&x));

    let x = 1; // default i32

    println!("int: {} size_of_val: {}", x, size_of_val(&x));

    let x: i64 = 1;

    println!("int: {} size_of_val: {}", x, size_of_val(&x));

    let x = 1.0; // default f64

    println!("float: {} size_of_val: {}", x, size_of_val(&x));

    let x: f32 = 1.0;

    println!("float: {} size_of_val: {}", x, size_of_val(&x));

    let x: isize = 1; // by OS, I run in 64bit OS is like i64; I run in 32bit OS is like i32

    println!("int: {} size_of_val: {}", x, size_of_val(&x));

    // Literals and operators
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // Overflow error
    // println!("1 - 2 = {}", 1u32 - 2);

    // Boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);

    let y: Option<String> = Some("123".to_string());

    println!("{}", y.iter().count());
}
