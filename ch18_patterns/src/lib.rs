#[cfg(test)]
mod tests {
    use crate::{Point, Message, Color};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn syntax() {
        // matching literals
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            _ => println!("others"),
        }

        // matching named variables
        let x = Some(5);
        match x {
            Some(y) => println!("y = {:?}", y),
            _ => println!("others"),
        }

        // multiple pattern
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            _ => println!("others"),
        }

        // matching ranges of values
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("others"),
        }
    }

    #[test]
    fn destruct() {
        // destructuring structs
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p; // a and b are variables
        let Point { x, y } = p; // x and y are variables
        match p { // destruct with literal values
            Point { x, y: 0 } => println!("y is zero"),
            Point { x: 0, y } => println!("x is zero"),
            Point { x, y } => println!("others")
        }

        // destructuring enums
        // mentioned before, like Option<T>, Result<T, E>

        // destructuring nested structs and enums
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb");
            Message::ChangeColor(Color::Hsv(g, s, v)) => println!("hsv"),
            _ => println!("others"),
        }

        // destructuring structs and tuples
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    #[test]
    fn ignore() {
        // ignoring an entire value with _
        fn foo(_: i32, y: i32) {}

        // ignoring parts of a value with a nested _
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => println!("numbers"),
        }

        // ignoring an unused variable by starting its name with _
        let _x = 5; // no compile warning even though _x is not used anywhere

        // ignoring remaining parts of a value with ..
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => println!("first and last"),
        }
    }

    #[test]
    fn guard() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("x < 5"),
            Some(x) => println!("other x"),
            _ => println!("others"),
        }
    }

    #[test]
    fn at() {
        enum Msg {
            Hello { id: i32 },
        }

        let msg = Msg::Hello { id: 5 };
        match msg {
            Msg::Hello { id: id_var @ 3..=7, } => println!("id: {}", id_var),
            Msg::Hello { id: 10..=12 } => println!("10 to 12"),
            Msg::Hello { id } => println!("other"),
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}


