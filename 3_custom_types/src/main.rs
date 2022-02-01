use activity::Person;

use crate::activity::{
    inspect, rect_area, square, List, Operations, Pair, Point, Rectangle, Unit, WebEvent, LANGUAGE,
    THRESHOLD,
};

mod activity;

fn enum_scope() {
    use crate::activity::Status::{Poor, Rich};

    let status = Rich;

    match status {
        Rich => println!("Rich {}", Rich as i32),
        Poor => println!("Poor {}", Poor as i32),
    }
}

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

    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let press = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste(String::from("test"));
    let click = WebEvent::Click { x: 10, y: 5 };

    inspect(load);
    inspect(unload);
    inspect(press);
    inspect(paste);
    inspect(click);

    let x = Operations::Add;

    println!("{}", x.run(1, 2));

    enum_scope();

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(4);
    list = list.prepend(7);

    println!("len {} ", list.len());
    println!("total {}", list.stringify());

    println!("Constant {} {}", LANGUAGE, THRESHOLD);
}
