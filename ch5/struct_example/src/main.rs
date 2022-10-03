struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//tuple struct
struct Color(i32,i32,i32);
struct Point(i32, i32, i32);

//unit-like struct


fn main() {
    let user1 = User {
        username: String::from("Gargamel"),
        email: String::from("gargamel@email.com"),
        sign_in_count: 1,
        active: true,
    };

    // let user2 = User {
    //     email: String::from("smurf@email.com"),
    //     username: String::from("Smurf"),
    //     sign_in_count: user1.sign_in_count,
    //     active: user1.active,
    // };

    //struct update syntax

    let user2 = User {
        email: String::from("smurf@email.com"),
        username: String::from("Smurf"),
        ..user1
    };


}
