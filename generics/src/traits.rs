use std::fmt::{Debug, Display};
use std::iter::Sum;

pub trait Summary {
    fn summarize(&self) -> String;

    // with default implementation
    fn summarize2(&self) -> String {
        String::from("Read more...")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// traits as parameters
pub fn notify(item: &impl Summary) {}

// trait bound syntax, same as notify
pub fn notify2<T: Summary>(item: &T) {}

// multiple traits
pub fn notify3(item: &(impl Summary + Display)) {}

// trait bound with + syntax, same as notify3
pub fn notify4<T: Summary + Display>(item: &T) {}

// where syntax
fn f1<T, U>(t: &T, u: &U) -> ()
    where T: Display + Clone,
          U: Clone + Debug {}

// return traits
fn f2() -> impl Summary {
    Tweet {
        username: "a".to_string(),
        content: "b".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{Summary, Tweet};

    #[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("happy"1),
        };

        let s = tweet.summarize();
    }
}