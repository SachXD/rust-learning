struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1: User = User {
        email: String::from("sach@mail.com"),
        username: String::from("sach123"),
        active: true,
        sign_in_count: 1,
    };

    let name: String = user1.username;
    user1.username = String::from("sugma123");

    let user2 = build_user(String::from("konoha@mail.com"), String::from("konoha123"));

    let user3: User = User {
        email: String::from("mock@mail.com"),
        username: String::from("mock123"),
        ..user2
    };
}

fn main1() {
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
//struct implementation -- its bizzare to understand it theoretically
