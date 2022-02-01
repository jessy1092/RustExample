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

pub enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("press {}", c),
        WebEvent::Paste(s) => println!("paste {}", s),
        WebEvent::Click { x, y } => println!("click at ({}, {})", x, y),
    }
}

pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

pub type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    pub fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

pub enum Status {
    Rich,
    Poor,
}

pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> List {
        Self::Nil
    }

    pub fn prepend(self, elm: u32) -> List {
        Self::Cons(elm, Box::new(self))
    }

    pub fn len(&self) -> i32 {
        match *self {
            Self::Cons(_, ref tail) => 1 + tail.len(), // recursive
            Self::Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Self::Cons(head, ref tail) => format!("({}, {})", head, tail.stringify()), // recursive
            Self::Nil => format!("Nil"),
        }
    }
}

pub static LANGUAGE: &str = "Rust";
pub const THRESHOLD: i32 = 10;
