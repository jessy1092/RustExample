mod activity;

use crate::activity::Complex;

fn main() {
    println!("Hello, world!");

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
