use std::io;

type Operation = fn(f64, f64) -> f64;

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        f64::NAN
    }
}

fn perform_operation(op: Operation, a: f64, b: f64) -> f64 {
    op(a, b)
}

fn main() {
    loop {
        println!("Select operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "5" => {
                println!("Calculator exiting.");
                break;
            }
            "1" | "2" | "3" | "4" => {
                let num1: f64 = loop {
                    let mut input = String::new();
                    println!("Enter first number:");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    match input.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Invalid input. Please enter a valid number."),
                    }
                };

                let num2: f64 = loop {
                    let mut input = String::new();
                    println!("Enter second number:");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    match input.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Invalid input. Please enter a valid number."),
                    }
                };

                let operation: Operation = match choice.trim() {
                    "1" => add,
                    "2" => subtract,
                    "3" => multiply,
                    "4" => divide,
                    _ => unreachable!(),
                };

                let result = perform_operation(operation, num1, num2);
                println!("Result: {}", result);
            }
            _ => println!("Invalid Input"),
        }
    }
}
