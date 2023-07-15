fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User::build_user(String::from("user2@email.com"), String::from("user2"));

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user3 {:?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black {:?}", black);
    println!("origin {:?}", origin);

    let subject = AlwaysEqual;

    println!("subject {:?}", subject);
}

impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
