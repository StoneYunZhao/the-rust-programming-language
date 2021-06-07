#[cfg(test)]
mod tests {
    #[test]
    fn box_() {
        let b = Box::new(5);
        println!("b = {}", b);
    }
}

