use std::mem::size_of;
use std::mem::size_of_val;

use crate::activity::reverse;
use crate::activity::transpose;
use crate::activity::Matrix;

mod activity;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

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
    println!("true && false is {}", true && false);
    println!("true || false is {}", true || false);
    println!("!true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("long tuple first value: {}", long_tuple.0);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8, -2i16), -3i32);

    println!("tuple of tuples first value: {}", tuple_of_tuples.0 .0);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    println!("long tuple: {:?}", long_tuple);

    let pair = (1, true);

    println!("pair: {:?}", pair);
    println!("reverse pair: {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let (destructured_a, destructured_b) = reverse(pair);

    println!("destructured tuple: {} {}", destructured_a, destructured_b);

    let m = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix: \n{}", m);
    println!("Transpose Matrix: \n{}", transpose(m));

    let y: Option<String> = Some(123.to_string());

    println!("{} {:?}", y.iter().count(), y.iter().next());

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("number of elements in array: {}", xs.len());

    println!("array occupies {} bytes", size_of_val(&xs));

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..7]);
}
