mod vec2d;

use crate::vec2d::Vec2D;

fn main() {
    let a = Vec2D::new(1.0, 2.0);
    let b = Vec2D::new(3.0, 4.0);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a.clone() + b.clone());
    println!("a - b = {}", a.clone() - b.clone());
    println!("a * 2 = {}", a.clone() * 2.0);
    println!("a · b = {}", a.dot(&b));
}
