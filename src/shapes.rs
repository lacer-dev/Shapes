#![allow(dead_code)]
// use reikna::{func, integral::*};
use core::f32;
use std::usize;

mod util {
    pub fn min_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
        if a <= b && a <= c {
            return a;
        } else if b <= a && b <= c {
            return b;
        } else {
            return c;
        }
    }

    pub fn mid_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
        if a > b {
            if b > c {
                return b;
            } else if a > c {
                return c;
            } else {
                return a;
            }
        } else {
            if a > c {
                return a;
            } else if b > c {
                return c;
            } else {
                return b;
            }
        }
    }

    pub fn max_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
        if a >= b {
            if a >= c {
                return a;
            } else {
                return c;
            }
        } else {
            if b >= c {
                return b;
            } else {
                return c;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::{f32, usize};
        use super::*;
        
        #[test] 
        fn test_min_basic() {
            assert_eq!(min_of_three(4, 1, 9), 1);
        }

        #[test] 
        fn test_min_with_large_number() {
            assert_eq!(min_of_three(-999999999, 0, 999), -999999999);
        }

        #[test]
        fn test_mid_with_f32() {
            assert_eq!(mid_of_three(4.3, -1.0, f32::consts::PI), f32::consts::PI);
        }

        #[test]
        fn test_mid_with_usize_limits() {
            assert_eq!(mid_of_three(usize::MIN, 0, usize::MAX), 0);
        }

        #[test]
        fn test_max_with_f32() {
            assert_eq!(max_of_three(f32::consts::E, 0.111111, 0.111111), f32::consts::E);
        }

        #[test]
        fn test_max_with_high_precision_f64() {
            assert_eq!(max_of_three(-333.333333333333333_f64, -222.222222222222222_f64, -111.111111111111111_f64), -111.111111111111111_f64);
        }
        
        const TEST_ARRAYS: [[usize; 3]; 6] = [
            [1, 2, 3], 
            [1, 3, 2], 
            [2, 1, 3], 
            [2, 3, 1], 
            [3, 1, 2],
            [3, 2, 1]];

        #[test]
        fn min_of_three_with_permutations() {
            for comb in TEST_ARRAYS {
                let [x, y, z] = comb;
                assert_eq!(min_of_three(x, y, z), 1);
            }
        }
        
        #[test]
        fn mid_of_three_with_permutations() {
            for comb in TEST_ARRAYS {
                let [x, y, z] = comb;
                assert_eq!(mid_of_three(x, y, z), 2);
            }
        }
        
        #[test]
        fn max_of_three_with_permutations() {
            for comb in TEST_ARRAYS {
                let [x, y, z] = comb;
                assert_eq!(max_of_three(x, y, z), 3);
            }
        }
    }
}

pub trait ShapeLike {
    fn num_sides(&self) -> usize;
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

pub trait TrapezoidLike: QuadrilateralLike {
    fn height(&self) -> f32;
    fn long_base_length(&self) -> f32;
    fn short_base_length(&self) -> f32;
    fn long_leg_length(&self) -> f32;
    fn short_leg_length(&self) -> f32;
}

pub trait TriangleLike: ShapeLike {
    fn side1_length(&self) -> f32;
    fn side2_length(&self) -> f32;
    fn side3_length(&self) -> f32;

    fn short_side_length(&self) -> f32 {
        util::min_of_three(
            self.side1_length(), 
            self.side2_length(), 
            self.side3_length())
    }

    fn mid_side_length(&self) -> f32 {
        util::mid_of_three(
            self.side1_length(), 
            self.side2_length(), 
            self.side3_length())
    }

    fn long_side_length(&self) -> f32 {
        util::max_of_three(
            self.side1_length(), 
            self.side2_length(), 
            self.side3_length())
    }

    fn is_equilateral(&self) -> bool {
        let l2 = self.side2_length();
        self.side1_length() == l2 && l2 == self.side3_length()
    }

    fn is_scalene(&self) -> bool {
        let l2 = self.side2_length();
        let l1 = self.side1_length();
        let l3 = self.side3_length();
        !(l1 == l2 || l2 == l3 || l3 == l1)  
    }

    fn is_isoceles(&self) -> bool {
        let l2 = self.side2_length();
        let l1 = self.side1_length();
        let l3 = self.side3_length();

        let mut count = 0;
        for c in [l1 == l2, l2 == l3, l3 == l1] {
            if c {
                count += 1;
            }
        }

        if count >= 2 {
            return true
        } else {
            return false
        }
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

#[derive(PartialEq, Clone)]
pub struct Point {
    x: f32,
    y: f32
}

/// A polygon with a variable number of vertices and sides.
#[derive(PartialEq, Clone)]
pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn from_points(points: &Vec<Point>) -> Self {
        Self {
            points: points.clone()
        }
    }
    
    pub fn line(&self, index: usize) -> f32 {
        let i2: usize = Polygon::wrap_index(index, self.points.len());
        let x1 = self.points[index].x;
        let y1 = self.points[index].y;
        let x2 = self.points[i2].x;
        let y2 = self.points[i2].y;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    fn wrap_index(i: usize, length: usize) -> usize {
        if i == length { 0 } else { i + 1 }
    }
}

impl ShapeLike for Polygon {
    fn num_sides(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self) -> f32 {
        (0..self.num_sides()).map(|i| self.line(i)).sum()
    }

    fn area(&self) -> f32 {
        let f = |i1: usize| -> f32 {
            let i2 = Polygon::wrap_index(i1, self.points.len());
            let p1 = &self.points[i1];
            let p2 = &self.points[i2];
            (p1.y + p2.y) * (p1.x - p2.x)
        };
        0.5 * (0..self.points.len()).map(f).sum::<f32>()
    }
}


/// A shape with a perimeter, an area, and any number of sides.
#[derive(PartialEq, Clone)]
pub struct Shape {
    sides: usize,
    perimeter: f32,
    area: f32,
}

impl Shape {
    pub fn new(sides: usize, perimeter: f32, area: f32) -> Self {
        Self {
            sides,
            perimeter,
            area
        }
    }
}

impl ShapeLike for Shape {
    fn num_sides(&self) -> usize { self.sides }
    fn perimeter(&self) -> f32 { self.perimeter }
    fn area(&self) -> f32 { self.area }
}

/// A rectangle.
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
    fn num_sides(&self) -> usize {
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

/// A square.
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
    fn num_sides(&self) -> usize { 4 }
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

// A trapezoid.
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
    fn num_sides(&self) -> usize {
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
    fn height(&self) -> f32 { self.height }
    fn long_base_length(&self) -> f32 { self.base1.max(self.base2) }
    fn short_base_length(&self) -> f32 { self.base1.min(self.base2) }
    fn long_leg_length(&self) -> f32 { self.leg1.max(self.leg2) }
    fn short_leg_length(&self) -> f32 { self.leg1.min(self.leg2) }
}

/// A triangle.
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
    fn num_sides(&self) -> usize {
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

/// A scalene triangle.
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
    fn num_sides(&self) -> usize { self.tri.num_sides() }
    fn perimeter(&self) -> f32 { self.tri.perimeter() }
    fn area(&self) -> f32 { self.tri.area() }
}

impl TriangleLike for ScaleneTriangle {
    fn side1_length(&self) -> f32 { self.tri.side1_length() }
    fn side2_length(&self) -> f32 { self.tri.side2_length() }
    fn side3_length(&self) -> f32 { self.tri.side3_length() }
}

impl ScaleneTriangleLike for ScaleneTriangle {}

/// An isoceles triangle
#[derive(PartialEq, Clone)]
pub struct IsocelesTriangle {
    base_length: f32,
    sides_length: f32
}

impl IsocelesTriangle {
    /// Constructs a new `IsocelesTriangle` with `base_length` as the base's length and `sides_length` as the length of the other two sides.
    pub fn new(base_length: f32, sides_length: f32) -> Self {
        Self {
            base_length, 
            sides_length
        }
    }
}

impl ShapeLike for IsocelesTriangle {
    fn num_sides(&self) -> usize { 
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

/// An equilateral triangle.
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
    fn num_sides(&self) -> usize { 
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

/// An ellipse.
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
    fn num_sides(&self) -> usize {
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

/// A circle.
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
    fn num_sides(&self) -> usize {
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