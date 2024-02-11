//Here’s a small programming problem: 
//  Write a function that takes a string of words separated by spaces 
//  Returns the first word it finds in that string.
//  If the function doesn’t find a space in the string, the entire string should be returned.

fn first_word(s:&String) -> usize{
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return i;
        }
    }
    s.len()
}

fn _first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut second_word_start: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if second_word_start == 0 {
                second_word_start = i+1; 
                continue;
            };
            return &s[second_word_start..i];
            }
        }
    &s[second_word_start..]
}
fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    s.clear();

    // A string slice is a reference to part of a String, and it looks like this:
    // the slice data structure stores the starting position and the length of the slice,
    // which corresponds to ending_index minus starting_index
    let mut  s_1 = String::from("hello world");

    // let _hello = &s_1[0..5];
    // let _world = &s_1[6..11];
    // So now we can call the fixed first word function
    let word = _first_word(&s_1);
    let s_word = second_word(&s_1);
    
    println!("Second Word: {s_word}, First Word: {word}");
    s_1.clear();
}
