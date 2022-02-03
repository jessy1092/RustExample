fn main() {
    let x = 5;

    println!("x = {}", x);

    let (x, y) = (1, 2);

    println!("x = {} y = {}", x, y);

    let x: i32 = 10;

    println!("x = {}", x);

    let mut x = 5;
    println!("x = {}", x);

    x = 15;
    println!("x = {}", x);

    let shadow_binding = 1;

    {
        println!("Inner Shadow Binding {}", shadow_binding);

        let shadow_binding = 3;

        println!("Inner Shadow Binding {}", shadow_binding);
    }
    println!("Outsider Shadow Binding {}", shadow_binding);

    let shadow_binding = 2;
    println!("Outsider Shadow Binding {}", shadow_binding);

    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding {}", a_binding);

    let mut _mutable_binding = 2;
    {
        let _mutable_binding = _mutable_binding;

        // Error
        // _mutable_binding = 20;
        println!("Mutable Binding {}", _mutable_binding)
    }

    _mutable_binding = 3;

    println!("Mutable Binding {}", _mutable_binding)
}
