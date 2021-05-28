fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved into s2


    println!("s1: {}", s1) // compile error because s1 is invalid
}
