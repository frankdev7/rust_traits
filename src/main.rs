use std::{fmt::Debug, hash::Hash};

trait HasArea {
    fn area(&self) -> f64;
    fn is_larger(&self, other: &Self) -> bool;
}

trait AproxEqual {
    fn aprox_equal(&self, other: &Self) -> bool;
}

impl AproxEqual for f32 {
    fn aprox_equal(&self, other: &Self) -> bool {
        (self - other).abs() < 1.0
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14*(self.radius*self.radius)
    }
    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

struct Square {
    x: f64,
    y: f64
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.x * self.y
    }
    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}


fn main() {
    let c1 = Circle { x: 1.0, y: 2.0, radius: 1.4 };
    let c2 = Circle { x: 2.0, y: 3.0, radius: 4.0 };
    let s = Square { x: 2.0, y: 3.0 };
    print_area(c1);
    print_area(s);
    // println!("Is c1 grather than c2: {}", c1.is_larger(&c2));

    let r = Rectangle { x: 1, y: 2, width: 2, height: 2 };
    println!("Is square: {}", r.is_square());

    println!("Is aprox equal: {}", 3.2.aprox_equal(&3.0));
    
    let p = Person { name: "frank".to_string(), surname: "c r".to_string() };
    println!("Foo: {:?}", p);
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn foo<T: Clone + Debug> (x: T ){
    x.clone();
}

fn foo_where<T, K> (x: T, y: K) 
    where T: Clone + Debug,
          K: Clone {

}

#[derive(Debug)]
struct Person {
    name: String,
    surname: String
}