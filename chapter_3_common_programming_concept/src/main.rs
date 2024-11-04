fn main() {

    shadowing();
    mutable_constants()
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

