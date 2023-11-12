struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
}
