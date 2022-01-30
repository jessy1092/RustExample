fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1 // Notice: It doesn't have semicolon at the end.
}

fn add_two(x: i32) -> i32 {
    return x + 2;
}

fn main() {

    print_number(5);

    print_sum(5, 6);

    println!("{}", add_one(4));

    println!("{}", add_two(4));
}
