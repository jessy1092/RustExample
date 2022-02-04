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
