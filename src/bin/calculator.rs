use std::io;

fn main() {
    let (first_number, second_number, operation) = read_input();

    match operation.as_str() {
        "+" => println!("Result: {}", add(first_number, second_number)),
        "-" => println!("Result: {}", subtract(first_number, second_number)),
        "*" => println!("Result: {}", multiply(first_number, second_number)),
        "/" => match divide(first_number, second_number) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        },
        _ => println!("Invalid operation"),
    }
}

fn add(first_number: f64, second_number: f64) -> f64 {
    first_number + second_number
}

fn subtract(first_number: f64, second_number: f64) -> f64 {
    return first_number - second_number;
}

fn multiply(first_number: f64, second_number: f64) -> f64 {
    return first_number * second_number;
}

fn divide(first_number: f64, second_number: f64) -> Result<f64, String> {
    if second_number == 0.0 {
        Err(String::from("Division by zero is not allowed."))
    } else {
        Ok(first_number / second_number)
    }
}

fn read_input() -> (f64, f64, String) {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let first_number = input.trim().parse().expect("Please enter a valid number.");
    input.clear();

    println!("Enter operation (+, -, *, /):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let operation = input.trim().to_string();
    input.clear();

    println!("Enter second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let second_number = input.trim().parse().expect("Please enter a valid number.");

    return (first_number, second_number, operation);
}
