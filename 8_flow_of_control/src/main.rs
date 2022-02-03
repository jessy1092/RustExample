use std::fmt::Write;

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", increase");

        n * 10
    } else {
        println!("halve the number");

        n / 2
    };

    println!("{} -> {}", n, big_n);

    let mut count = 0_u32;

    println!("loop");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("end");
            break;
        }
    }

    'outer: loop {
        println!("enter outer loop");

        'inner: loop {
            println!("Enter inner loop");

            // exit inner
            // break;

            // break outer
            break 'outer;
        }

        // unnecessary
        // break;
    }

    println!("Exited the outer loop");

    let mut loop_counter = 0;

    let result = loop {
        loop_counter += 1;

        if loop_counter == 10 {
            break loop_counter * 2;
        }
    };

    assert_eq!(result, 20);
    assert_eq!(loop_counter, 10);

    let mut n = 1;

    while n < 31 {
        if n % 15 == 0 {
            println!("fuzzbuzz");
        } else if n % 3 == 0 {
            println!("fuzz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    for n in 1..=30 {
        if n % 15 == 0 {
            println!("fuzzbuzz");
        } else if n % 3 == 0 {
            println!("fuzz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Lee", "Jessy"];

    for name in names.iter() {
        match name {
            &"Lee" => println!("Lee is here"),
            _ => println!("hello {}", name),
        }
    }

    println!("names: {:?}", names);

    for name in names.into_iter() {
        match name {
            "Lee" => println!("Lee is here"),
            _ => println!("hello {}", name),
        }
    }

    // Failed, already moved
    // println!("names: {:?}", names);

    let mut names = vec!["Lee", "Jessy"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Lee" => "Lee is here",
            _ => "hello", // TODO: How to dynamic format
        }
    }

    println!("mut names: {:?}", names);

    let number = 13;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is prime"),
        13..=19 => println!("A teen"),
        _ => println!("Else"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    let tuple = (1, -2, 3);

    match tuple {
        (0, y, z) => println!("First 0 other: {} {}", y, z),
        (1, ..) => println!("First 1"),
        _ => println!("Else"),
    }

    let reference = &4;

    match reference {
        &val => println!("Get value {}", val),
    }

    match *reference {
        val => println!("Get value without reference {}", val),
    }

    let is_not_reference = 5;

    match is_not_reference {
        ref r => println!("Display not reference with reference {:p}", r),
    }

    let mut mut_val = 6;

    match mut_val {
        ref mut m => {
            *m += 10;
            println!("add 10 {:?}", m);
        }
    }

    println!("Change {}", mut_val);

    struct Foo {
        x: (i32, i32),
        y: i32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("x is 1, b: {}, y: {}", b, y),
        Foo {
            y: 2,
            x: change_name,
        } => println!("y is 2, x: {:?}", change_name),
        Foo { y, .. } => println!("y is {:?}", y),
    }

    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("Same"),
        (x, y) if x + y == 0 => println!("abs"),
        (x, _) if x % 2 == 1 => println!("add"),
        _ => println!("Else"),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("{} > zero", i),
        _ => println!("not posiible"),
    }

    let age = 11;

    match age {
        0 => println!("birth"),
        n @ 1..=12 => println!("child {:?}", n),
        n => println!("old {:?}", n),
    }

    let some = Some(13);

    match some {
        Some(n @ 1..=12) => println!("specific {}", n),
        Some(n @ 13..=25) => println!("large layer {}", n),
        _ => println!("else"),
    }

    if let Some(n) = some {
        println!("get number from Some {}", n);
    }

    let none_number: Option<i32> = None;

    if let Some(n) = none_number {
        println!("Not possible {}", n);
    } else {
        println!("None")
    }

    enum Bar {
        Baz,
        Qux(u32),
    }

    let a = Bar::Baz;
    let b = Bar::Qux(10);

    if let Bar::Baz = a {
        println!("a is bar");
    }

    if let Bar::Qux(n @ 1..=12) = b {
        println!("Binding {}", n);
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("end");
            optional = None;
        } else {
            println!("{}", i);
            optional = Some(i + 1);
        }
    }
}
