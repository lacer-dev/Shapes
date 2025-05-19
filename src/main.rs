use shapes::*;
pub mod shapes;

fn main() {
    let shape: Shape = Shape::new(10, 15.0, 23.0);

    println!("Sides     = {}", shape.num_sides());
    println!("Perimeter = {}", shape.perimeter());
    println!("Area      = {}", shape.area());
}