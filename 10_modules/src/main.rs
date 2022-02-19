mod folder;

mod my_mod {
    fn private_function() {
        println!("called my_mod::private_function()");
    }

    pub fn function() {
        println!("called my_mod::function()");
    }

    pub fn indirect_access() {
        println!("called my_mod::indirect_access()");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        // the same as private
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested()");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called my_mod::nested::public_function_in_my_mod(), that");
            public_function_in_nested();
        }

        pub(super) fn public_function_in_supper_mod() {
            println!("called my_mod:nested::public_function_in_supper_mod()");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called my_mod::call_public_function_in_my_mod() that");
        nested::public_function_in_my_mod();
        println!("> ");
        nested::public_function_in_supper_mod();
        println!("> ");
        private_nested::restricted_function_in_my_mod();
        private_nested::restricted_function();
        private_nested::function();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate()");
    }

    mod private_nested {
        pub fn function() {
            println!("called my_mode::private_nested::function()");
        }

        // Cannot use on whole crate
        pub(crate) fn restricted_function() {
            println!("called my_mod::private_nested::restricted_function()");
        }

        pub(in crate::my_mod) fn restricted_function_in_my_mod() {
            println!("called my_mod::private_nested::restricted_function_in_my_mod()");
        }
    }
}

fn function() {
    println!("called `function()`");
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents: contents }
        }

        pub fn get_contents(&self) -> &T {
            &self.contents
        }
    }
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod extend {
    fn function() {
        println!("called extend::function()");
    }

    mod cool {
        pub fn function() {
            println!("called extend::cool::function()");
        }
    }

    pub fn indirect_call() {
        self::function();
        function();

        self::cool::function();

        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();

    // Cannot call here
    // my_mod::nested::public_function_in_my_mod();

    // Error
    // my_mod::private_nested::restricted_function();

    let open_box = my::OpenBox {
        contents: "public information",
    };

    println!("The open box contains: {}", open_box.contents);

    let _close_box = my::CloseBox::new("classified information");

    println!("The close box get contents: {}", _close_box.get_contents());

    use deeply::nested::function as other_function;

    other_function();

    println!("Entering block");
    {
        use crate::deeply::nested::function;

        function();

        println!("Leaving block");
    }

    function();

    extend::indirect_call();

    folder::function();

    folder::indirect_access();

    folder::nested::function();
}
