#[derive(Debug)] // auto implments Debug trait to print
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // this is like static method
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // this is method
    // & of self is required.
    // if self doesn't have &, the ownership of self will be move to this argument.
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        email: String::from(("some@example.com")),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };
    let mut user1 = User {
        email: String::from(("some@example.com")),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("updated@example.com");
    println!("{:?}", user1);
    // :#? can show formatted struct
    println!("{:#?}", user1);
    let user1 = build_user(String::from("email"), String::from("name2"));
    println!("{:#?}", user1);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
