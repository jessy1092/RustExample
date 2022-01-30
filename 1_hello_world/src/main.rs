mod activity;

use crate::activity::{Color, Complex, List};

fn main() {
    println!("Hello, world!");

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![1, 2, 3]);

    println!("{}", v);

    let colors = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    for color in colors.iter() {
        println!("{:}", *color);
    }
}
