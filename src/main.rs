/*example that covers generics, traits and lifetimes all in one function */
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_word = "call";

    let response = longest_with_an_announcement(&novel, &first_word, "Breaking news!");
    
    println!("Response is: {}", response);
}
