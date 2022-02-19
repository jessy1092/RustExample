use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::activity::{
    blue, difference, red, BlueJay, Cardinal, Container, Contains, Inch, Length, Mm, Turkey,
};

mod activity;

struct A;
struct SingleGen<T>(T);

fn generic_function<T>(_s: SingleGen<T>) {}

struct GenVal<T> {
    gen_val: T,
}

// Specific type
// impl GenVal<char> {
//     fn value(&self) -> &char {
//         println!("{}", &self.gen_val);

//         &self.gen_val
//     }
// }

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

struct Triangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug {:?}", t);
    println!("Display {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self))
    }
}

struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

fn main() {
    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);

    generic_function::<char>(SingleGen('c'));
    generic_function(SingleGen('a'));

    let y = GenVal { gen_val: 3i32 };

    println!("{}", y.value());

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // Error vec not implement Display
    // printer(vec![1]);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("{}", red(&cardinal));
    println!("{}", blue(&blue_jay));

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);

    vec.print_in_option();

    let age = Years(5);
    let age_days = age.to_days();

    println!("Old {}", old_enough(&age));
    println!("Old {}", old_enough(&age_days.to_years()));

    let years_as_primitive_1 = age.0;
    let Years(years_as_primitive_2) = age; // Destructuring

    let n1 = 3;
    let n2 = 10;

    let container = Container(n1, n2);

    println!("{}", container.contains(&n1, &n2));

    println!("difference is {}", difference(&container));

    let _tupl1: PhantomTuple<char, i32> = PhantomTuple('Q', PhantomData);
    let _tupl2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meeters = one_meter + one_meter;

    println!("{:?}", two_feet.0);
    println!("{:?}", two_meeters.0);

    // Error
    // let one_feter = one_foot + one_meter;
}
