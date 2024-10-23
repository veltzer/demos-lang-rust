use std::io;

/// Performs the specified arithmetic operation on two numbers
fn calculate(num1: f64, num2: f64, op: &str) -> Result<f64, &'static str> {
    match op {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Division by zero!")
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operation!")
    }
}

fn main() {
    println!("Simple Calculator");
    println!("Enter first number:");
    
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read first number");
    
    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
    println!("Enter operation (+, -, *, /):");
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read operation");
    
    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read second number");
    
    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
    match calculate(num1, num2, op.trim()) {
        Ok(result) => println!("{} {} {} = {}", num1, op.trim(), num2, result),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculations() {
        assert_eq!(calculate(5.0, 3.0, "+").unwrap(), 8.0);
        assert_eq!(calculate(5.0, 3.0, "-").unwrap(), 2.0);
        assert_eq!(calculate(5.0, 3.0, "*").unwrap(), 15.0);
        assert_eq!(calculate(6.0, 2.0, "/").unwrap(), 3.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(calculate(5.0, 0.0, "/").is_err());
    }

    #[test]
    fn test_invalid_operation() {
        assert!(calculate(5.0, 3.0, "%").is_err());
    }
}
