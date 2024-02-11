fn main() {
    // the action of creating a reference is called borrowing
    let s1 = String::from("hello");
    // this is how you pass a reference to the function
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    
    // This is how you declare and use a mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    // NOTE: if you have a mutable reference to a value, you can´t have other references to that value.
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // We can use curly brackets to create a new scope, 
    // allowing for multiple mutable references, just not simultaneous ones:
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    
    // Also you cannot have a mutable reference while having an immutable one to the same value.
}
// this is how you define a reference parameter in a function
// instead of taking ownership of the value:
fn calculate_length(s: &String) -> usize {
    // the value it points to will not be dropped when the reference stops being used.
    s.len()
    // Just as variables are immutable by default, so are references. 
    // We’re not allowed to modify something we have a reference to.
    // That means that this code, i.e would not work.
    //      s.push_str(", world");
}

fn change(some_string: &mut String) {
    // We can modify a borrowed value with a mutable reference
    some_string.push_str(", world");
}
