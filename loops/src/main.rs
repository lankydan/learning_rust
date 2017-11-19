fn main() {
    // infinite loop
    // loop {
    //   println!("again");
    // }
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1; // cant do number--  ??
    }
    println!("done");

    let array = [1,2,3,4,5];
    
    for element in array.iter() {
        println!("The value is: {}", element);
    }

    // countdown instead of while look
    for number in (1..4).rev() {
      println!("{}", number);
    }
}
