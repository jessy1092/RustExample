use crate::activity::{Pair, Point, Rectangle};

mod activity;

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

    fizzbuzz_to(30);

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter {}", rectangle.perimeter());
    println!("Rectangle area {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 2.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // pair.destroy();

    // Already move
    // println!("pair {:?}", pair)
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz")
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}
