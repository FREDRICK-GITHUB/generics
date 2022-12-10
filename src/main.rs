
struct Point<T> {
    x: T,
    y: T,
}

/* example implementation of generic types in methods */
impl<T> Point<T> {
    fn x(&self)-> &T {
        &self.x
    }
}


fn main() {
    let p = Point { x:5, y:10 };
    println!("p.x = {}", p.x());
}
