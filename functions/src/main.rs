fn main() {
    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("y: {}", y);

    println!("five: {}", five());

    println!("plus: {}", plus_one(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
