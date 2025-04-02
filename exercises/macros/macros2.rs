// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    ($val:expr) => {
        println!("Check out my macro: {}", $val);
    };
}

fn main() {
    my_macro!("Hello, world!");
}
