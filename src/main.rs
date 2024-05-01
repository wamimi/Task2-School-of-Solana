#![allow(dead_code)]
use std::fmt;
mod tests;

// ------------------------------------------------------------------------------------------------
// Traits
//
pub trait Shape {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}
// ------------------------------------------------------------------------------------------------
// Structs
//
pub struct Calculator {
    pub x: i64,
    pub y: i64,
}

pub struct Rectangle {
    a: f64,
    b: f64,
}

pub struct Circle {
    r: f64,
}
// ------------------------------------------------------------------------------------------------
// Non-Trait implementations for Structs
//
impl Calculator {
    pub fn new(arg1: &i64, arg2: &i64) -> Self {
        Self { x: *arg1, y: *arg2 }
    }

    pub fn addition(&self) -> Option<i64> {
        self.x.checked_add(self.y)
    }

    pub fn subtraction(&self) -> Option<i64> {
        self.x.checked_sub(self.y)
    }

    pub fn multiplication(&self) -> Option<i64> {
        self.x.checked_mul(self.y)
    }

    pub fn division(&self) -> Option<i64> {
        if self.y == 0 {
            None
        } else {
            self.x.checked_div(self.y)
        }
    }
}

impl Rectangle {
    pub fn try_new(arg1: &f64, arg2: &f64) -> Result<Self, &'static str> {
        if *arg1 >= 0.0 && *arg2 >= 0.0 {
            Ok(Self { a: *arg1, b: *arg2 })
        } else {
            Err("Rectangle sides must be greater or equal to zero!")
        }
    }

    pub fn set_a(&mut self, a: &f64) -> Result<(), &'static str> {
        if *a >= 0.0 {
            self.a = *a;
            Ok(())
        } else {
            Err("Side a must be greater or equal to zero!")
        }
    }

    pub fn set_b(&mut self, b: &f64) -> Result<(), &'static str> {
        if *b >= 0.0 {
            self.b = *b;
            Ok(())
        } else {
            Err("Side b must be greater or equal to zero!")
        }
    }

    pub fn get_a(&self) -> f64 {
        self.a
    }

    pub fn get_b(&self) -> f64 {
        self.b
    }
}

impl Circle {
    pub fn try_new(arg1: &f64) -> Result<Self, &'static str> {
        if *arg1 >= 0.0 {
            Ok(Self { r: *arg1 })
        } else {
            Err("Circle radius must be greater or equal to zero!")
        }
    }

    pub fn set_r(&mut self, r: &f64) -> Result<(), &'static str> {
        if *r >= 0.0 {
            self.r = *r;
            Ok(())
        } else {
            Err("Radius must be greater or equal to zero!")
        }
    }

    pub fn get_r(&self) -> f64 {
        self.r
    }
}

// ------------------------------------------------------------------------------------------------
// Trait Shape implementations for Structs
//
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }

    fn circumference(&self) -> f64 {
        2.0 * (self.a + self.b)
    }
}

// TODO: Implement the Shape trait and its methods for the Circle struct
// Hint: you can use std::f64::consts::PI

// ------------------------------------------------------------------------------------------------
// Examples
//
fn calculator_example() {
    let x_in: i64 = -53;
    let y_in: i64 = 17;
    let calculator = Calculator::new(&x_in, &y_in);
    println!("{calculator}");
}

fn rectangle_example() {
    let a_in: f64 = 7.0;
    let b_in: f64 = 3.0;
    let mut rectangle = Rectangle::try_new(&a_in, &b_in).unwrap();
    println!("{rectangle}");

    let new_a_in: f64 = 15.0;
    let new_b_in: f64 = 2.0;

    let res = rectangle.set_a(&new_a_in);
    assert!(res.is_ok());

    let res = rectangle.set_b(&new_b_in);
    assert!(res.is_ok());

    println!("{rectangle}");

    let new_b_in: f64 = -2.0;
    let res = rectangle.set_b(&new_b_in);
    assert_eq!(
        res.err(),
        Some("Side b must be greater or equal to zero!")
    );
}

fn circle_example() {
    let r_in: f64 = 17.0;
    let mut circle = Circle::try_new(&r_in).unwrap();
    println!("{circle}");

    let new_r_in: f64 = 15.0;
    let res = circle.set_r(&new_r_in);
    assert!(res.is_ok());
    println!("{circle}");

    let new_r_in: f64 = -15.0;
    let res = circle.set_r(&new_r_in);
    assert_eq!(
        res.err(),
        Some("Radius must be greater or equal to zero!")
    );
}

// ------------------------------------------------------------------------------------------------
// Main
//
fn main() {
    calculator_example();
    rectangle_example();
    circle_example();
}

// Implementing this trait for a type will automatically implement the ToString trait for the type,
// allowing the usage of the .to_string() method. Prefer implementing the Display trait for a type,
// rather than ToString. More info here: https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out =
            "-----------------------------------------------------------------\n".to_string();
        out.push_str("Printing Calculator results\n");
        out.push_str(&format!(
            "{} + {} = {}\n",
            self.x,
            self.y,
            self.addition().unwrap()
        ));
        out.push_str(&format!(
            "{} - {} = {}\n",
            self.x,
            self.y,
            self.subtraction().unwrap()
        ));
        out.push_str(&format!(
            "{} * {} = {}\n",
            self.x,
            self.y,
            self.multiplication().unwrap()
        ));
        out.push_str(&format!(
            "{} / {} = {}\n",
            self.x,
            self.y,
            self.division().unwrap()
        ));
        out.push_str("-----------------------------------------------------------------\n");
        write!(f, "{}", out)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out =
            "-----------------------------------------------------------------\n".to_string();
        out.push_str("Printing Rectangle\n");
        out.push_str(&format!(
            "a:{}, b:{}, area:{}, circumference:{}\n",
            self.get_a(),
            self.get_b(),
            self.area(),
            self.circumference(),
        ));
        out.push_str("-----------------------------------------------------------------\n");
        write!(f, "{}", out)
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out =
            "-----------------------------------------------------------------\n".to_string();
        out.push_str("Printing Circle\n");
        out.push_str(&format!(
            "r:{}, area:{}, circumference:{}\n",
            self.get_r(),
            self.area(),
            self.circumference(),
        ));
        out.push_str("-----------------------------------------------------------------\n");
        write!(f, "{}", out)
    }
}
