fn main() {

    shadowing();
    mutable_constants();
    tuples();
}
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn mutable_variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("the value of the constant is:{THREE_HOURS_IN_SECONDS}")
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
fn mutable_constants() {
    let mut spaces = "   ";
    //spaces = spaces.len(); can't reassign because of type mismatch
    let size = spaces.len();
    println!("The size of spaces is: {size}");
}

fn data_types(){
    let guess: u32 = "42".parse().expect("Not a number!");

        let x = 2.0; // f64

        let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10; //i32

    // subtraction
    let difference = 95.5 - 4.3; //f64

    // multiplication
    let product = 4 * 30; //i32

    // division
    let quotient = 56.7 / 32.2; //f64
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; //i32
}
fn tuples(){
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("the value of five_hundred is: {five_hundred}");
    let five_hundred = 499;
    println!("the value of five_hundred is: {five_hundred}");
    let x=(five_hundred,7.2,2);
    let six_point_four = x.1;
    println!("the value of six_point_four is: {six_point_four}");




}
