use std::io;

/// Calculates the letter grade based on a numerical score
fn calculate_grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'X' // Invalid score
    }
}

/// Returns feedback based on the letter grade
fn get_feedback(grade: char) -> &'static str {
    match grade {
        'A' => "Excellent work!",
        'B' => "Good job!",
        'C' => "Satisfactory.",
        'D' => "Needs improvement.",
        'F' => "Please seek additional help.",
        _ => "Invalid grade."
    }
}

fn main() {
    println!("Grade Calculator");
    println!("Enter the numerical score (0-100):");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let score: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
    if score > 100 {
        println!("Invalid score! Score must be between 0 and 100.");
        return;
    }
    
    let grade = calculate_grade(score);
    println!("Score: {}, Grade: {}", score, grade);
    println!("{}", get_feedback(grade));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_calculation() {
        assert_eq!(calculate_grade(95), 'A');
        assert_eq!(calculate_grade(85), 'B');
        assert_eq!(calculate_grade(75), 'C');
        assert_eq!(calculate_grade(65), 'D');
        assert_eq!(calculate_grade(55), 'F');
    }

    #[test]
    fn test_grade_boundaries() {
        assert_eq!(calculate_grade(90), 'A');
        assert_eq!(calculate_grade(89), 'B');
        assert_eq!(calculate_grade(80), 'B');
        assert_eq!(calculate_grade(79), 'C');
        assert_eq!(calculate_grade(70), 'C');
        assert_eq!(calculate_grade(69), 'D');
        assert_eq!(calculate_grade(60), 'D');
        assert_eq!(calculate_grade(59), 'F');
    }

    #[test]
    fn test_feedback() {
        assert_eq!(get_feedback('A'), "Excellent work!");
        assert_eq!(get_feedback('B'), "Good job!");
        assert_eq!(get_feedback('C'), "Satisfactory.");
        assert_eq!(get_feedback('D'), "Needs improvement.");
        assert_eq!(get_feedback('F'), "Please seek additional help.");
        assert_eq!(get_feedback('X'), "Invalid grade.");
    }
}
