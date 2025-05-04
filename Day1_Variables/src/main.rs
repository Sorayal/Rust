
/**
The usage of variables.
let has it´s origins in Mathematics
"Let x = 2 ..."

syntax:
let (mut) variableName: datatype = value;

i8 : Integer with 8 bit width

*/

fn main() {
    println!("Simple calculations!");

    let a:i16 = 10;
    let b:i16 = 5;

    // mut is necessary, variables and references are immutable by default, except using mut
    let mut result:i16 = add(a, b);
    println!("The result is: {result}");

    result = subtract(a, b);
    println!("The result is: {result}");
}

// return statement seems unnecessary
fn add(a:i16, b:i16) -> i16 {
    a + b
}

fn subtract(a:i16, b:i16) -> i16 {
    a - b
}
