struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@email.com");

    let user2 = User {
        email: String::from("user2@email.com"),
        ..user1 // fills other data as user1's
    };
    
    println!("Hello, world!");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}