// Project: variables
// Author: Greg Folker

fn main() {
    // By default, new variables are always immutable
    let x = 5;

    println!("The value of x is: {}", x);

    //x = 6; would be a compiler error "Cannot assign twice to immutable variable x"

    let mut y = 5;

    println!("The value of y is: {}", y);

    // Allowed because the variable is mutable
    y = 6;

    println!("The value of y is now: {}", y);

    // 'mut' cannot be used with constants
    // Constants aren't just immutable by default - they are always immutable
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // 'Shadowing' is when you declare a new variable with the same name
    // as a previous variable. The new variable shadows the previous variable
    // meaning the second variable's value is what appears when the variable is used
    //
    // The advantage of shadowing is it allows you to perform a few transformations
    // on a value but have the variable remain immutable afterwards
    //
    // This is accomplished using the 'let' keyword
    let x = 5;

    let x = x + 1; // x is now 6

    let x = x * 2; // x is now 12

    println!("The value of x is: {}", x);

    // Another example of shadowing
    // 'spaces' is initially a string but then shadowed to an int
    // This means we don't need separate variables 'spaces_str' and 'spaces_num'
    // if we only care about the value for 'spaces_num' in our program
    let spaces = "      ";
    let spaces = spaces.len();

    println!("There were {} spaces in spaces", spaces)

    // This would be a compiler error "mismatched types" since there was no shadowing
    // let spaces = "      ";
    // spaces = spaces.len();
}
