fn main() {
    let x = 5_u32;

    let y = {
        let x_squared = x * x;
        let y_cube = x_squared * x;

        x_squared + y_cube + x
    };

    let z = {
        x * 2;
    };

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}
