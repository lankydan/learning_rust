fn main() {
    let number = 3;

    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }

    let condition = true;
    let number_1 = if condition { // if is an expression and therefore can be used on the RHS of a let statement
      5
    } else {
      // no semi colons for expressions return value, otherwise it becomes a statement and does not return a value
      "six" // this will fail because both arms of the if/else statements must have the same value types
      // this is because Rust needs to know the type of number_1 at compile time to check that it is being used correctly everywhere in the code
    };

    println!("The value of the number is: {}", number_1);
}
