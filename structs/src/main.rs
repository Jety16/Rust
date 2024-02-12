
// We used the owned String type rather than the &str string slice type. 
// This is a deliberate choice because we want each instance of this struct to own all of its data
// and for that data to be valid for as long as the entire struct is valid.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs: 
// Tuple structs have the added meaning the struct name provides 
// but don’t have names associated with their field
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User {
    User {active: true, username, email, sign_in_count:1,}
}


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("somerusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // This is how you use a value
    println!("{}", user1.username);
    user1.sign_in_count = 2;
    
    // Here we are just using the build_user function!
    let mut user2 = build_user(String::from("pepe@gmail.com"), String::from("username"));


    // The ..var notation let us use data from other struct of the same type!
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    
    // Note that the black and origin values are different 
    // types because they’re instances of different tuple structs. 
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
