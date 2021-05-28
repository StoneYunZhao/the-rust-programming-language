fn main() {
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        // struct update syntax: the remaining fields not explicitly set
        // should have the same value as the fields in the given instance.
        ..user1
    };

    // tuple struct
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: string) -> User {
    User {
        email, // field init shorthand syntax
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    username: String,
    // field
    email: String,
    sign_in_count: u64,
    active: bool,
}