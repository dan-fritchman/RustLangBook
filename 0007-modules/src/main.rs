
mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function () {}
    mod inside {
        pub fn inner_function(){}
        fn secret_function() {}
    }
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    println!("Hello, world!");

    outermost::middle_function();
    // outermost::middle_secret_function();
    // outermost::inside::inner_function();
    // outermost::inside::secret_function();

    use a::series::of;
    of::nested_modules();
    use a::series::of::*;
    nested_modules();
}
