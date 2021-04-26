struct User {
    username: String,
    password: String,
    email: String,
    sign_in_count: u32,
    activate: bool,
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

