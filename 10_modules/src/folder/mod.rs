mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called folder::function()");
}

fn private_function() {
    println!("called folder::private_function()");
}

pub fn indirect_access() {
    println!("called folder::indirect_access()");

    private_function();
}
