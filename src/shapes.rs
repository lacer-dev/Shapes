#![allow(dead_code)]
use core::f32;
use util::
// use reikna::{func, integral::*};

pub trait ShapeLike {
    fn num_sides(&self) -> u32;
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;

    fn semiperimeter(&self) -> f32 {
        self.perimeter() / 2.0
    }
}

pub trait QuadrilateralLike: ShapeLike {
    fn diagonal1(&self) -> f32;
    fn diagonal2(&self) -> f32;
    
    fn long_diagonal(&self) -> f32 {
        self.diagonal1().max(self.diagonal2())
    }

    fn short_diagonal(&self) -> f32 {
        self.diagonal1().min(self.diagonal2())
    }
}

pub trait RectangleLike: QuadrilateralLike {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    
    fn diagonal(&self) -> f32 {
        (self.width().powi(2) + self.height().powi(2)).sqrt()
    }
    
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

    fn short_side_length(&self) -> f32 {
        min_of_three(self.side1_length(), self.side2_length(), self.side3_length())
    }

    fn mid_side_length(&self) -> f32 {
        mid_of_three(self.side1_length(), self.side2_length(), self.side3_length())
    }

    fn long_side_length(&self) -> f32 {
        max_of_three(self.side1_length(), self.side2_length(), self.side3_length())
    }
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
    fn radius_minor(&self) -> f32;
    /// The length of the bigger radius of the ellipse
    fn radius_major(&self) -> f32;
    /// The circumference of the ellipse
    fn circumference(&self) -> f32;
    /// The eccentricity of the ellipse
    fn eccentricity(&self) -> f32 {
        (1.0 - self.radius_minor().powi(2) / self.radius_major().powi(2)).sqrt()
    }
}

pub trait CircleLike: EllipseLike {
    fn radius(&self) -> f32;
    
}

pub trait TrapezoidLike: QuadrilateralLike {
    fn height(&self) -> f32;
    fn long_base_length(&self) -> f32;
    fn short_base_length(&self) -> f32;
    fn long_leg_length(&self) -> f32;
    fn short_leg_length(&self) -> f32;
}

/// Shape
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

/// Rectangle
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

impl QuadrilateralLike for Rectangle {
    fn diagonal1(&self) -> f32 {
        self.diagonal()
    }
    
    fn diagonal2(&self) -> f32 {
        self.diagonal()
    }
}

impl RectangleLike for Rectangle {
    fn width(&self) -> f32 { self.width }
    fn height(&self) -> f32 { self.height }
}

/// Square
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

    pub fn from_area(area: f32) -> Self {
        Self {
            side_length: area.sqrt()
        }
    }
}

impl ShapeLike for Square {
    fn num_sides(&self) -> u32 { 4 }
    fn perimeter(&self) -> f32 { 4.0 * self.side_length() }
    fn area(&self) -> f32 { self.side_length().powi(2) }
}

impl QuadrilateralLike for Square {
    fn diagonal1(&self) -> f32 {
        self.diagonal()
    }
    
    fn diagonal2(&self) -> f32 {
        self.diagonal()
    }    
}

impl RectangleLike for Square {
    fn width(&self) -> f32 { self.side_length }
    fn height(&self) -> f32 { self.side_length }
    fn is_square(&self) -> bool { true }
    fn diagonal(&self) -> f32 {
        (self.side_length.powi(2) + self.side_length.powi(2)).sqrt()
    }
}

impl SquareLike for Square {
    fn side_length(&self) -> f32 { self.side_length }
}

/// Triangle
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

/// ScaleneTriangle
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

/// IsocelesTriangle
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

/// EquilateralTriangle
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

/// Ellipse
#[derive(PartialEq, Clone)]
struct Ellipse {
    radius1: f32,
    radius2: f32
}

impl Ellipse {
    pub fn new(radius1: f32, radius2: f32) -> Self {
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
        f32::consts::PI * self.radius_major() * self.radius_minor()
    }
}

impl EllipseLike for Ellipse {
    fn radius_major(&self) -> f32 {
        self.radius1.max(self.radius2)
    }

    fn radius_minor(&self) -> f32 {
        self.radius1.min(self.radius2)
    }

    /// Not implemented yet
    fn circumference(&self) -> f32 {
        // let e_squared = self.eccentricity().powi(2) as f64;
        // let f = |theta: f64| (1.0 - e_squared * theta.sin().powi(2)).sqrt();
        // 4.0 * self.radius_major() * integrate(&func!(f), 0.0, f64::consts::PI / 2.0) as f32
        0.0
    }
}

/// Circle
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
    fn radius_major(&self) -> f32 {
        self.radius()
    }

    fn radius_minor(&self) -> f32 {
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

// Trapezoid
#[derive(PartialEq, Clone)]
struct Trapezoid {
    height: f32,
    base1: f32,
    base2: f32,
    leg1: f32,
    leg2: f32
}

impl Trapezoid {
    pub fn new_isoceles(height: f32, base1: f32, base2: f32) -> Self {
        let legs_length = Trapezoid::legs_from_height_and_bases(height, base1, base2);
        Self {
            height,
            base1,
            base2,
            leg1: legs_length,
            leg2: legs_length
        }
    }

    pub fn new_scalene(base1: f32, base2: f32, leg1: f32, leg2: f32) -> Self {
        Self {
            height: Trapezoid::height_from_sides(base1, base2, leg1, leg2),
            base1,
            base2,
            leg1,
            leg2,
        }
    }

    fn area_from_height_and_bases(h: f32, b1: f32, b2: f32) -> f32 {
        (b1 + b2) * h / 2.0
    }
    
    fn area_from_sides(b1: f32, b2: f32, l1: f32, l2: f32) -> f32 {
        let a = b1.min(b2);
        let b = b1.max(b2);
        let multiplier = (a + b) / (4.0 * (b - a).abs());
        let multiplicand = ((-a + b + l1 + l2) 
            * (a - b + l1 + l2) 
            * (a - b + l1 - l2) 
            * (a - b - l1 + l2)).sqrt();
        multiplier * multiplicand
    }

    fn legs_from_height_and_bases(h: f32, b1: f32, b2: f32) -> f32 {
        (h.powi(2) + ((b1 - b2) / 2.0).powi(2)).sqrt()
    }

    fn height_from_sides(a: f32, b: f32, c: f32, d: f32) -> f32 {
        let p = a + b + c + d;
        let numerator_squared = (p - 2.0 * a) 
            * (p - 2.0 * b) 
            * (p - 2.0 * b - 2.0 * d) 
            * (p - 2.0 * b - 2.0 * c);
        let numerator = numerator_squared.sqrt();
        let denominator = 2.0 * (b - a).sqrt();
        numerator / denominator
    }
}
 
impl ShapeLike for Trapezoid {
    fn num_sides(&self) -> u32 {
        return 4;
    }

    fn perimeter(&self) -> f32 {
        self.base1 + self.base2 + self.leg1 + self.leg2
    }

    fn area(&self) -> f32 {
        Trapezoid::area_from_height_and_bases(self.height, self.base1, self.base2)
    }
}

impl QuadrilateralLike for Trapezoid {
    fn diagonal1(&self) -> f32 {
        let a = self.short_base_length();
        let b = self.long_base_length();
        let c = self.leg1;
        let d = self.leg2;
        let numerator = a * b.powi(2) - a.powi(2) * b - a * c.powi(2) + b * d.powi(2);
        let denominator = b - a;
        (numerator / denominator).sqrt()
    }
    
    fn diagonal2(&self) -> f32 {
        let a = self.short_base_length();
        let b = self.long_base_length();
        let c = self.leg1;
        let d = self.leg2;
        let numerator = a * b.powi(2) - a.powi(2) * b - a * d.powi(2) + b * c.powi(2);
        let denominator = b - a;
        (numerator / denominator).sqrt()
    }
}

impl TrapezoidLike for Trapezoid {
    fn height(&self) -> f32 {
        self.height
    }

    fn long_base_length(&self) -> f32 {
        self.base1.max(self.base2)   
    }

    fn short_base_length(&self) -> f32 {
        self.base1.min(self.base2)
    }

    fn long_leg_length(&self) -> f32 {
        self.leg1.max(self.leg2)
    }

    fn short_leg_length(&self) -> f32 {
        self.leg1.min(self.leg2)
    }
}