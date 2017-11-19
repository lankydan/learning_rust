use std::io;

fn main() {
    loop {  
        println!("Fibonacci number generator");
        println!("Enter number n");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("oops...");

        let n: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                if input.trim() == "quit" {
                  break;
                }
                println!("Just enter a number next time!");
                continue;
            }
        };

        println!("The fibonacci number of {} is {}", n, fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}