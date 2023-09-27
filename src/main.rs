use std::io;
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, &'static str> { 
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                return Err("Cannot divide by zero");
            }
            Ok(a / b)
        },
    }
}

fn main() {
    let mut buffer = String::new();

    println!("Enter first number: ");
    io::stdin().read_line(&mut buffer).expect("Failed to read line");  
    let a = buffer.trim_end().parse::<f64>().unwrap_or_else(|_| buffer.parse::<i64>().expect("Failed to parse number") as f64);
    buffer.clear();

    println!("Enter a operation id (1: Add, 2: Subtract, 3: Multiply, 4: Divide): ");
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let operation_id = buffer.trim_end().parse::<i32>().expect("Failed to parse number");
    buffer.clear();

    println!("Enter second number: ");
    io::stdin().read_line(&mut buffer).expect("Failed to read line"); 
    let b = buffer.trim_end().parse::<f64>().unwrap_or_else(|_| buffer.parse::<i64>().expect("Failed to parse number") as f64);
    buffer.clear();

    let operation = match operation_id {
        1 => Some(Operation::Add(a, b)),
        2 => Some(Operation::Subtract(a, b)),
        3 => Some(Operation::Multiply(a, b)),
        4 => Some(Operation::Divide(a, b)),
        _ => None,
    };

    match operation {
        Some(operation) => {
            if let Ok(res) = calculate(operation) {
                println!("Result: {}", res);
            } else {
                println!("Invalid operation");
            }
        }
        None => println!("Invalid operation"),
    } 
}