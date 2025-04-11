use std::io;

fn main() {
    // Take user input
    let mut input = String::new();
    println!("Enter an expression (e.g., 9-4-2+4):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Trim whitespace
    let input = input.trim();

    // Vectors to store numbers and operators
    let mut numbers: Vec<i32> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    // Temporary string to build numbers
    let mut temp = String::new();

    // Loop through each character in the input
    for c in input.chars() {
        if c.is_digit(10) {
            // If the character is a digit, add it to temp
            temp.push(c);
        } else if !temp.is_empty() {
            // Convert temp to a number, store it, and clear temp
            numbers.push(temp.parse().unwrap());
            temp.clear();

            // If the character is an operator, store it
            if "+-*/".contains(c) {
                operators.push(c);
            }
        } else if "+-*/".contains(c) {
            // Store the operator in case there's no preceding number in temp
            operators.push(c);
        }
    }

    // Add the last number in temp (if any) to the list
    if !temp.is_empty() {
        numbers.push(temp.parse().unwrap());
    }

    // Print the extracted numbers and operators
    println!("Extracted numbers: {:?}", numbers);
    println!("Extracted operators: {:?}", operators);

    // Perform the operations using the extracted numbers and operators
    if !numbers.is_empty() {
        let mut result = numbers[0]; // Start with the first number

        for (i, &op) in operators.iter().enumerate() {
            if let Some(&num) = numbers.get(i + 1) {
                match op {
                    '+' => result += num,
                    '-' => result -= num,
                    '*' => result *= num,
                    '/' => result /= num,
                    _ => (), // Ignore invalid operators
                }
            }
        }

        println!("Result of the expression: {}", result);
    } else {
        println!("No numbers to operate on!");
    }
}
