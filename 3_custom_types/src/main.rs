use activity::Person;

use crate::activity::{rect_area, square, Pair, Point, Rectangle, Unit};

mod activity;

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };

    let bottom_right = Point { x: 5.2, ..point };

    println!("point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: Point {
            x: bottom_right.x,
            y: bottom_right.y + 10.0,
        },
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    let Pair(integer, decimal) = pair;

    println!("Pair ({}, {})", integer, decimal);

    println!(
        "Rectangle Area: ({}, {}) ({}, {}) {}",
        rectangle.top_left.x,
        rectangle.top_left.y,
        rectangle.bottom_right.x,
        rectangle.bottom_right.y,
        rect_area(&rectangle)
    );

    println!(
        "Point {:?} width: {} Square {:?}",
        point,
        5.0,
        square(&point, 5.0)
    );
}
