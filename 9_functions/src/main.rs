use std::vec;

use crate::activity::{
    apply, apply_to_3, call_me, create_fn, create_fnmut, create_fnonce, foo, function, is_odd,
    some_fn, sum_odd_numbers, Pair, Point, Rectangle,
};

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

    fn func(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("func {}", func(i));
    println!("closure annotated {}", closure_annotated(i));
    println!("closure inferred {}", closure_inferred(i));

    let one = || 1;
    println!("one {}", one());

    let color = String::from("green");

    let print = || print!("color: {}", color);

    print();

    let _reborrow = &color;

    print();

    let _color_moved = color;

    // color is move
    // print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    let consume = || {
        print!("movable: {:?}", movable);
        drop(movable);
    };

    consume();
    // Error movable already drop
    // consume();

    let heystack = vec![1, 2, 3];

    let contains = move |needle| heystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // already move into closure
    // println!("Vec ? {}", heystack.len());

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // for Fn
        println!("I said {}", greeting);

        // for FnMut
        farewell.push_str("!!!!");
        println!("Then {}", farewell);

        // for FnOnce
        // drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    let closure = || println!("I'm a closure");

    call_me(closure);
    call_me(function);

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // as the same as
    // println!("Find 2 in vec1: {:?}", iter.find(|x| **x == 2));

    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
    // as the same as
    // println!("Find 2 in vec2: {:?}", into_iter.find(|x| *x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];

    let index = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index, Some(5));

    println!("Find the sum of all the squared odd number under 1000");

    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared > upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    let sum: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);

    println!("functional styles: {}", sum);

    // foo();

    let a: () = some_fn();
    println!("This function return and you can see this line");

    println!("Sum of odd numbers up to 9: {}", sum_odd_numbers(9));
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
