use core::f32;

pub trait ShapeLike {
    fn num_sides(&self) -> u32;
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

pub trait RectangleLike: ShapeLike {
    fn width(&self) -> f32;
    fn height(&self) -> f32;

    fn is_square(&self) -> bool {
        self.width() == self.height()
    }
}   

pub trait SquareLike: RectangleLike {
    fn side_length(&self) -> f32;
}

pub trait TriangleLike: ShapeLike {
    fn side1_length(&self) -> f32;
    fn side2_length(&self) -> f32;
    fn side3_length(&self) -> f32;
}

pub trait ScaleneTriangleLike: TriangleLike {
   
}

pub trait IsocelesTriangleLike: TriangleLike {
    fn base_length(&self) -> f32;
    fn sides_length(&self) -> f32;
}

pub trait EquilateralTriangleLike: TriangleLike {
    fn side_length(&self) -> f32;
}

pub trait EllipseLike: ShapeLike {
    /// The length of the smaller radius of the ellipse
    fn radius_small(&self) -> f32;
    /// The length of the bigger radius of the ellipse
    fn radius_big(&self) -> f32;
    /// The circumference of the ellipse
    fn circumference(&self) -> f32;
}

pub trait CircleLike: EllipseLike {
    fn radius(&self) -> f32;
}

#[derive(PartialEq, Clone)]
pub struct Shape {
    sides: u32,
    perimeter: f32,
    area: f32,
}

impl Shape {
    pub fn new(sides: u32, perimeter: f32, area: f32) -> Self {
        Self {
            sides,
            perimeter,
            area
        }
    }
}

impl ShapeLike for Shape {
    fn num_sides(&self) -> u32 { self.sides }
    fn perimeter(&self) -> f32 { self.perimeter }
    fn area(&self) -> f32 { self.area }
}

#[derive(PartialEq, Clone)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }
}

impl ShapeLike for Rectangle {
    fn num_sides(&self) -> u32 {
        4
    }

    fn perimeter(&self) -> f32 {
        2.0 * self.width() + 2.0 * self.height()
    }

    fn area(&self) -> f32 {
        self.width() * self.height()
    }
}

impl RectangleLike for Rectangle {
    fn width(&self) -> f32 { self.width }
    fn height(&self) -> f32 { self.height }
}

#[derive(PartialEq, Clone)]
pub struct Square {
    side_length: f32
}

impl Square {
    pub fn new(side_length: f32) -> Self {
        Self {
            side_length
        }
    }
}

impl ShapeLike for Square {
    fn num_sides(&self) -> u32 { 4 }
    fn perimeter(&self) -> f32 { 4.0 * self.side_length() }
    fn area(&self) -> f32 { self.side_length().powi(2) }
}

impl RectangleLike for Square {
    fn width(&self) -> f32 { self.side_length }
    fn height(&self) -> f32 { self.side_length }
    fn is_square(&self) -> bool { true }
}

impl SquareLike for Square {
    fn side_length(&self) -> f32 { self.side_length }
}

#[derive(PartialEq, Clone)]
pub struct Triangle {
    side1: f32,
    side2: f32,
    side3: f32,
}

impl Triangle {
    pub fn new(side1: f32, side2: f32, side3: f32) -> Self {
        Self {
            side1,
            side2,
            side3
        }
    }
}

impl ShapeLike for Triangle {
    fn num_sides(&self) -> u32 {
        3
    }

    fn perimeter(&self) -> f32 {
        self.side1_length() + self.side2_length() + self.side3_length()
    }

    fn area(&self) -> f32 {
        let s: f32 = self.perimeter() / 2.0;
        (s * (s - self.side1_length()) * (s - self.side2_length()) * (s - self.side3_length())).sqrt()
    }
}

impl TriangleLike for Triangle {
    fn side1_length(&self) -> f32 { self.side1 }
    fn side2_length(&self) -> f32 { self.side2 }
    fn side3_length(&self) -> f32 { self.side3 }
}

#[derive(PartialEq, Clone)]
pub struct ScaleneTriangle {
    tri: Triangle,
}

impl ScaleneTriangle {
    pub fn new(side1: f32, side2: f32, side3: f32) -> Self {
        Self {
            tri: Triangle::new(side1, side2, side3)
        }
    }
}

impl ShapeLike for ScaleneTriangle {
    fn num_sides(&self) -> u32 { self.tri.num_sides() }
    fn perimeter(&self) -> f32 { self.tri.perimeter() }
    fn area(&self) -> f32 { self.tri.area() }
}

impl TriangleLike for ScaleneTriangle {
    fn side1_length(&self) -> f32 { self.tri.side1_length() }
    fn side2_length(&self) -> f32 { self.tri.side2_length() }
    fn side3_length(&self) -> f32 { self.tri.side3_length() }
}

impl ScaleneTriangleLike for ScaleneTriangle {}

#[derive(PartialEq, Clone)]
pub struct IsocelesTriangle {
    base_length: f32,
    sides_length: f32
}

impl IsocelesTriangle {
    pub fn new(base_length: f32, sides_length: f32) -> Self {
        Self {
            base_length, 
            sides_length
        }
    }
}

impl ShapeLike for IsocelesTriangle {
    fn num_sides(&self) -> u32 { 
        3 
    }
    
    fn perimeter(&self) -> f32 { 
        self.base_length() + self.sides_length() + self.sides_length() 
    }

    fn area(&self) -> f32 {
        (1.0 / 4.0) * self.base_length() * (4.0 * self.sides_length().powi(2) - self.base_length().powi(2)).sqrt()
    }
}

impl TriangleLike for IsocelesTriangle {
    fn side1_length(&self) -> f32 { self.base_length() }
    fn side2_length(&self) -> f32 { self.sides_length() }
    fn side3_length(&self) -> f32 { self.sides_length() }
}

impl IsocelesTriangleLike for IsocelesTriangle {
    fn base_length(&self) -> f32 { self.base_length }
    fn sides_length(&self) -> f32 { self.sides_length }
}

#[derive(PartialEq, Clone)]
pub struct EquilateralTriangle {
    side: f32,
}

impl EquilateralTriangle {
    pub fn new(side_length: f32) -> Self {
        Self {
            side: side_length,
        }
    }
}

impl ShapeLike for EquilateralTriangle {
    fn num_sides(&self) -> u32 { 
        3 
    }

    fn perimeter(&self) -> f32 {
        3.0 * self.side_length()
    }

    fn area(&self) -> f32 {
        (3.0_f32).sqrt() / 4.0 * self.side_length().powi(2)
    }
}

impl TriangleLike for EquilateralTriangle {
    fn side1_length(&self) -> f32 { self.side_length() }
    fn side2_length(&self) -> f32 { self.side_length() }
    fn side3_length(&self) -> f32 { self.side_length() }
}

impl EquilateralTriangleLike for EquilateralTriangle {
    fn side_length(&self) -> f32 { self.side }
    
}

#[derive(PartialEq, Clone)]
struct Ellipse {
    radius1: f32,
    radius2: f32
}

impl Ellipse {
    fn new(radius1: f32, radius2: f32) -> Self {
        Self {
            radius1,
            radius2
        }
    }    
}

impl ShapeLike for Ellipse {
    fn num_sides(&self) -> u32 {
        0
    }

    fn perimeter(&self) -> f32 {
        self.circumference()
    }

    fn area(&self) -> f32 {
        f32::consts::PI * self.radius_big() * self.radius_small()
    }
}

impl EllipseLike for Ellipse {
    fn radius_big(&self) -> f32 {
        self.radius1.max(self.radius2)
    }

    fn radius_small(&self) -> f32 {
        self.radius1.min(self.radius2)
    }

    /// Not implemented yet
    fn circumference(&self) -> f32 {
        0.0
    }
}

#[derive(PartialEq, Clone)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn new(radius: f32) -> Self {
        Self {
            radius
        }
    }    
}

impl ShapeLike for Circle {
    fn num_sides(&self) -> u32 {
        0
    }

    fn perimeter(&self) -> f32 {
        self.circumference()
    }

    fn area(&self) -> f32 {
        f32::consts::PI * self.radius().powi(2)
    }
}

impl EllipseLike for Circle {
    fn radius_big(&self) -> f32 {
        self.radius()
    }

    fn radius_small(&self) -> f32 {
        self.radius()
    }

    fn circumference(&self) -> f32 {
        2.0 * f32::consts::PI * self.radius()
    }
}

impl CircleLike for Circle {
    fn radius(&self) -> f32 {
        self.radius
    }
}