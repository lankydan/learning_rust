use std::io;

fn main() {
  loop {
    // to use a unicode character it is a 'u' follows by curly brackets and its number
    // the below example represents the U+2109 unicode character
    // backslash is to escape it from the current string
    println!("Enter \u{2109} temperature");

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Oops... Something went wrong");

    let temperature: f64 = match input.trim().parse() {
      Ok(number) => number,
      Err(_) => {
        if input.trim() == "quit" {
          break;
        }
        println!("Enter a number...");
        continue;
      }
    };

    let output: f64 = (temperature - 32.0) * (5.0 / 9.0);
    println!("The temperature is {}\u{2103}", output);
  }
}
