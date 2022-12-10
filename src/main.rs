/*lifetime annotation for functions. the lifetime of the returned type is equivalent to the shortest lifetime of the function arguments/signature */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}


fn main() {
    let string1 =  String::from("long string is long");
    
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string between the two is {}", result);
    }
    
}
