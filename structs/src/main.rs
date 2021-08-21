fn main() {
    struct User {
        name: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User {
        name: String::from("thinceller"),
        email: String::from("hoge@example.com"),
        sign_in_count: 0,
        active: true,
    };
    let user2 = User {
        name: String::from("thinceller2"),
        email: String::from("fuga@example.com"),
        ..user1
    };
    println!(
        "{}, {}, {}, {}",
        user2.name, user2.email, user2.sign_in_count, user2.active
    );
}
