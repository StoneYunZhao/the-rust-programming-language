#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 3, 4);
        assert!(1 + 2 == 3);
        assert!(1 == 1, "assert with custom message: {}", 123)
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("asf")
    }

    #[test]
    #[should_panic(expected = "too long")]
    fn should_panic2() {
        panic!("too long a")
    }

    #[test]
    fn result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("failed"))
        }
    }
}
