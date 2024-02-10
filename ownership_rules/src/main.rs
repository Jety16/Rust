fn ownership_first(){
    let s = "Hello";  // New s comes in the scope
    // its all about the scope... like if i call the s out of this scope its not longer valid
    // trivial  
    {
        // You can create a String from a string literal using the from function, like so:  
        // When we call String::from, its implementation requests the memory it needs.
        let mut s = String::from(s);

        // the memory is automatically returned once the variable that owns it goes out of scope.
        s.push_str(", world");

        println!("{s}");
    }

    println!("{s}");
    
    // In this other case we have this example right here.
    let s1 = String::from("hello");
    let s2 = s1;

    // we dont copy the value of s1 and create a new heap entry nonono nothing like that
    // insteed we create a new entry in the stack with a pointer to the same value in the heap!
    // (A String is made up of three parts, 
    // a pointer to the memory that holds the contents of the string, 
    // a length, 
    // and a capacity.)
    // This group of data is stored on the stack
    // the memory on the heap that holds the contents.
    // pointer----> 0  | H
    //              1  | E
    //              2  | L
    //              3  | L
    //              4  | O

    // when s2 and s1 go out of scope, they will both try to free the same memory. 
    // causing  a double free error. To ensure memory safety, after the line let s2 = s1;, 
    // Rust considers s1 as no longer valid. 
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope. 
    // i tried to use s1 after s2 is created and doesn't work:


    // We can just clone the value if we want to reuse it
    // Here’s an example of the clone method in action:
    // Note that we are now shadowing the other s1 and s2!
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn main() {
    // Im learning the ownership rules :D 
    let s = String::from("hello");  // s comes into scope
    ownership_first();              // s is not in the scope anymore!

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


