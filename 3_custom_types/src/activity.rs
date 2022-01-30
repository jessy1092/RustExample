#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

pub struct Unit;

pub struct Pair(pub i32, pub f32);

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

fn abs(value: f32) -> f32 {
    if value >= 0.0 {
        return value;
    }

    value * -1.0
}

pub fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle {
        top_left,
        bottom_right,
    } = rectangle;

    let abs_x = abs(top_left.x - bottom_right.x);
    let abs_y = abs(top_left.y - bottom_right.y);

    abs_x * abs_y
}

pub fn square(point: &Point, width: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + width,
            y: point.y - width,
        },
    }
}
