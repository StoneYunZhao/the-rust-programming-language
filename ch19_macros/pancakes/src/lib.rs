#[cfg(test)]
mod tests {
    use crate::Pancakes;
    use hello_macro::HelloMacro;

    #[test]
    fn it_works() {
        Pancakes::hello_macro();
    }
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;
