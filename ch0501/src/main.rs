// Defining and Instantiating Structs

// struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// function that takes an email and username and returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct UserWithSliceType {
    // username: &str,
    // email: &str,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    // create an instance of the User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // change the value in the email field of a User instance
    user1.email = String::from("anotheremail@example.com");

    // create a new User instance using some of the values from user1
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        ..user1
    };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Hello, world!");
}
