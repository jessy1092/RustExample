#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

#[cfg(feature = "custom")]
fn conditional_function() {
    println!("condition met!!")
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!")
    } else {
        println!("Yes. It's definitely *not* linux!")
    }

    conditional_function();
}
