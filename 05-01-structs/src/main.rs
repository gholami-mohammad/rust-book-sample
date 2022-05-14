// struct with named fileds
#[derive(Debug)]// <- help programmers to debug the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs: tructs without named propery
struct Color(i32,i32, i32);

fn build_user(username: String, email:String)->User {
    return  User{
        username, // short hand syntax  <=> username: username
        email,
        active:true,
        sign_in_count: 1,
    };
}

fn main() {
    // 1- create immutable instance of struct
    let user1 = User{
        username: String::from("mgh"),
        email: String::from("mgh@example.com"),
        active: true,
        sign_in_count: 12,
    };
    println!("user1: {:?}", user1);

    let name = user1.username;

    // 2- mutable instance
    let mut user2 = User {
        username: String::from("gholami"),
        email: String::from("gholami@example.com"),
        active: true,
        sign_in_count: 19,
    };

    // now can change user2 properties
    user2.active = false;

    // 3- user build_user to create a user
    let user3 = build_user(String::from("sam"), String::from("sam@example.com"));

    // 4- create new instance using existing one
    let user4 = User{
        ..user3 // <- fill the user4 with all fields of user3
    };
    println!("user4.username: {}", user4.username);
    // all fields of user3 now are borrowed by user4 :)

    // 5- create new instance using existing one but borrow some fileds
    let user5 = User {
        username: String::from("nariman"),
        email: String::from("nariman@example.com"),
        ..user4 // <- active and sign_in_count will borrowed from user4
    };
    
    // 6- new instance of tuple struct
    let c = Color(128, 215, 32,);
    println!("Red:{} Green:{} Blue:{}", c.0, c.1, c.2);
}
