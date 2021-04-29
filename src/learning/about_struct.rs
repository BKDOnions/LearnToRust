struct User {
    username: String,
    password: String,
    email: String,
    sign_in_count: u32,
    activate: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
    area: u16,
}

impl Rectangle {
    fn area(&mut self) {
        self.area = self.height * self.width
    }
    fn can_contain(&self, another: Rectangle) -> bool {
        (self.height > another.height && self.width > another.width) || (self.width > another.height && self.height > another.width)
    }
}

impl Rectangle {
    fn square(side: u16) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
            area: side * side,
        }
    }
}

fn user_builder(username: String, email: String) -> User {
    User {
        username,
        password: password_gen(),
        email,
        sign_in_count: 1,
        activate: true,
    }
}

fn password_gen() -> String {
    String::from("hello").to_string()
}

fn area(dimensions: (i16, i16)) -> i16 {
    dimensions.0 * dimensions.1
}

// By using struct
fn using_struct_area(rectangle: &mut Rectangle) {
    rectangle.area = rectangle.height * rectangle.width
}

pub fn printing_traits() {
    let mut rectangle = Rectangle {
        width: 100,
        height: 10,
        area: 0,
    };
    using_struct_area(&mut rectangle);
    println!("by printing using '#': {:#?}", rectangle);
    println!("by printing w/o '#': {:?}", rectangle);
    println!("squire with side = 10: {:?}", Rectangle::square(10));
}