#[derive(Clone)]
struct User {
    username: String,
    active: bool,
}

fn main() {
    let u1 = User {
        username: String::from("u1"),
        active: true,
    };

    let u2 = User {
        username: String::from("u2"),
        ..u1
    };

    println!("u2.username: {}", u2.username);
    println!("u2.active: {}", u2.active);
    let new = update_user(&u2.clone());
}

fn update_user(user: &User) -> User {
    user.clone()
}
// fn update_user(user: &User) -> User {
//     if true {
//         User {
//             username: String::from("u3"),
//             ..*user
//         }
//     } else {
//         *user
//     }
// }
