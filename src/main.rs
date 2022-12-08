
 /* Example of a struct of generic type */

 #[derive(Debug)]
 struct Point<T> {
    x: T,
    y: T,
 }

 /*defination of a struct that has generics that could be of different types */
#[derive(Debug)]
 struct Point1<T, U> {
    x: T,
    y: U,
 }

fn main() {
    let integer = Point {x:4,y:9};
    let  float = Point { x:7.5, y:1.3};
    println!("Two structs: integer struct has {:?} and float struct has {:?}", integer,float);

    let both_integer = Point1 { x:5,y:7 };
    let both_float = Point1 { x: 5.6, y: 8.3 };
    let integer_and_float = Point1 { x:6, y:1.2 };
    println!("We have three different structs: {:?} {:?} {:?}", both_integer, both_float, integer_and_float);
}
