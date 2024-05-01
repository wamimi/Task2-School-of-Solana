## Rust Shapes Calculator
This Rust project implements a shapes calculator, including basic mathematical operations and shape calculations. It defines several structs representing different shapes and a calculator to perform arithmetic operations.

Features
Arithmetic operations (addition, subtraction, multiplication, division) with overflow and underflow resilience.
Calculation of area and circumference for rectangles and circles.
Constructor methods for creating instances of the Calculator, Rectangle, and Circle structs.
Setter methods for modifying the dimensions of rectangles and circles.
Tests for all implemented functionalities to ensure correctness and reliability.
Implemented Components
Calculator
The Calculator struct represents a simple calculator capable of performing arithmetic operations on two operands. It includes methods for addition, subtraction, multiplication, and division, with resilience to overflow and underflow.

Rectangle
The Rectangle struct represents a rectangle with two sides (a and b). It includes methods for calculating the area and circumference of the rectangle. The struct also provides setter methods to modify the dimensions of the rectangle.

Circle
The Circle struct represents a circle with a radius (r). Similar to the rectangle, it includes methods for calculating the area and circumference of the circle. It also provides a setter method to modify the radius of the circle.

Running the Tests
To run the tests for this project, use the following command:

bash
Copy code
cargo test
This command will compile the project and execute all the defined tests, ensuring that all implemented functionalities work as expected.

Examples
The project includes examples demonstrating the usage of the implemented functionalities:

calculator_example: Shows how to perform arithmetic operations using the Calculator struct.
rectangle_example: Demonstrates the creation and modification of rectangles.
circle_example: Demonstrates the creation and modification of circles.
Usage
To use this project in your Rust application, you can add it as a dependency in your Cargo.toml file:

toml

[dependencies]
shapes_calculator = "0.1.0"
Then, import the necessary components in your code and start using them:

rust
use shapes_calculator::{Calculator, Rectangle, Circle};

fn main() {
    // usage
    let calculator = Calculator::new(&5, &3);
    println!("Addition result: {}", calculator.addition().unwrap());

    let rectangle = Rectangle::try_new(&10.0, &5.0).unwrap();
    println!("Rectangle area: {}", rectangle.area());

    let circle = Circle::try_new(&7.0).unwrap();
    println!("Circle circumference: {}", circle.circumference());
