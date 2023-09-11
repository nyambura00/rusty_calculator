use std::io::{self, Write};

/// TASK: Create a simple calculator using Enums and Pattern Matching

fn main() {
    let mut first_value = String::new();
    let mut second_value = String::new();
    let mut calc_operation = String::new();

    // pick values from stdin
    print!("Enter the first value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut first_value).unwrap();

    print!("Enter the second value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut second_value).unwrap();

    print!("Enter the desired operation(Either add, subtract, multiply, divide): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut calc_operation).unwrap();

    // parse and compute values
    let first_value: f64 = first_value.trim().parse().unwrap();
    let second_value: f64 = second_value.trim().parse().unwrap();
    let calc_operation: Operation = match calc_operation.trim() {
        "add" => Operation::Add(first_value, second_value),
        "subtract" => Operation::Subtract(first_value, second_value),
        "multiply" => Operation::Multiply(first_value, second_value),
        "divide" => Operation::Divide(first_value, second_value),
        _ => panic!("Invalid operation")
    };

    let result = calculate(calc_operation);
    println!("The {:?} result of {} and {} is {}", &calc_operation, first_value, second_value, result);
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn calculate(values: Operation) -> f64 {
    match values {
        Operation::Add(value_one, value_two) => value_one + value_two,
        Operation::Subtract(value_one, value_two) => {
            value_one - value_two
        },
        Operation::Multiply(value_one, value_two) => {
            value_one * value_two
        },
        Operation::Divide(value_one, value_two ) => {
            if value_two == 0.0 {
                eprintln!("Division by zero is not allowed.");
                std::process::exit(1);
            }
            value_one / value_two
        }
    }
}