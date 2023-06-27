use std::io;
enum Operations {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

impl Operations {
    fn operate(&self, x: i32, y: i32) -> i32 {
        match self {
            Operations::Addition => x + y,
            Operations::Substraction => x - y,
            Operations::Multiplication => x * y,
            Operations::Division => x / y,
        }
    }
}

fn main() {
    let error_mesage = "Failed to read";
    println!("'a' for addition, 's' for substraction, 'm' for multiplication, 'd' for division");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(error_mesage);
    let input = input.trim();
    let determine_operation = match input {
        "a" => Operations::Addition,
        "s" => Operations::Substraction,
        "m" => Operations::Multiplication,
        "d" => Operations::Division,
        _ => {
            println!("Invalid operation");
            // Default operation Addition
            Operations::Addition
        }
    };
    println!("Write the first number: ");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect(error_mesage);
    println!("Write the second number: ");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect(error_mesage);
    let first_number = first_number.trim().parse().expect(error_mesage);
    let second_number = second_number.trim().parse().expect(error_mesage);
    let result = determine_operation.operate(first_number, second_number);
    println!("The result is {}", result);
}
