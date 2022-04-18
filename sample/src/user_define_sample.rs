// ユーザー定義型
struct User {
    name: String,
    pref: String,
    role: String,
    age: u32,
}

fn main() {
    let user_data = User {
        name: String::from("Ken"),
        pref: String::from("Tokyo"),
        role: String::from("Chief"),
        age: 28,
    };


    println!("<<< user Info >>>\nname: {user_name},\npref: {user_pref},\nrole: {user_role},\nage: {user_age}",
            user_name = &user_data.name,
            user_pref = &user_data.pref,
            user_role = &user_data.role,
            user_age = &user_data.age);
}