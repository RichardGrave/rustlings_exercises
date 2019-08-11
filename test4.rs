// test4.rs
// This test covers the sections:
// - Modules
// - Macros

#[macro_export]
macro_rules! my_macro {
    ($x:expr) =>  (format!("Hello {}",$x))
}

// Write a macro that passes the test! No hints this time, you can do it!

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
