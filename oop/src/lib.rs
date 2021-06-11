mod post;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub trait Draw {
    fn draw(&self);
}

pub struct Screen1 {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen1 {
    pub fn run(&self) {
        for component in self.components.iter(){
            component.draw();
        }
    }
}


pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
    where
        T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
