#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("alice@example.com"),
        ..user1
    };

    println!("user2: {:?}", user2);
}
