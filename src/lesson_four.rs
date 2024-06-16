use std::ptr::addr_of;

mod lesson_four {}
/*struct User1<` &a> {
    active: bool,
    username: &`a str,
    email: &`a str,
    sign_in_count: u64,
}

pub(crate) fn lesson4() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}*/

pub(crate) fn lesson3() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 1, 2);
    let origin = Point(3, 4, 5);

    println!("{:?}",black);
    println!("{:?}",origin);
}

pub(crate) fn lesson2() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

pub(crate) fn lesson1() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user1)
}


#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
