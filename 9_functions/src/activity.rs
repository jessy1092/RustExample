pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    pub fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

#[derive(Debug)]
pub struct Pair(pub Box<i32>, pub Box<i32>);

impl Pair {
    pub fn destroy(self) {
        let Pair(first, secound) = self;

        println!("Destroying Pair ({}, {})", first, secound);
    }
}

pub fn apply<F>(mut f: F)
where
    F: FnMut(),
{
    f()
}

pub fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn call_me<F: Fn()>(f: F) {
    f();
}

pub fn function() {
    println!("I'm a function");
}

pub fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

pub fn create_fnmut() -> impl FnMut() {
    let mut text = "FnMut".to_owned();

    move || {
        text.push_str("!!!!");
        println!("This is a: {}", text)
    }
}

pub fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || {
        println!("This is a: {}", text);
        drop(text);
    }
}

pub fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn foo() -> ! {
    panic!("This call never returns.")
}

pub fn some_fn() {
    ()
}

pub fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;

    for i in 0..up_to {
        let addition: u32 = match i % 2 == 1 {
            true => i,
            false => continue, // OK, match u32 because never return
        };
        acc += addition;
    }

    acc
}
