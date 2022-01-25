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

    let x: isize = 1; // by OS, I run in 64bit OS is like i64

    println!("int: {} size_of_val: {}", x, size_of_val(&x));

    let y: Option<String> = Some("123".to_string());

    println!("{}", y.iter().count());

}
