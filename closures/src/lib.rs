#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let c1 = |num| {
            num
        };

        let c2 = |num: u32| -> u32 {
            num
        };

        fn add_one_v1(x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| { x + 1 };
        let add_one_v4 = |x| x + 1;

        // compile error
        let example_closure = |x| x;
        let s = example_closure(String::from("hello"));
        let n = example_closure(5);

        // move keyword
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
    }
}

struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
}
