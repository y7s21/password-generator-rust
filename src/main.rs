use rand::Rng;
use std::io;
use std::collections::HashSet;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIAL_CHARACTERS: &str = "!@#$%^&*()_-+=<>?";

// Function to generate the password based on user options
fn generate_password(length: usize, include_uppercase: bool, include_numbers: bool, include_special: bool) -> String {
    let mut charset = LOWERCASE.to_string(); // Default is lowercase letters

    // Add more characters based on user preferences
    if include_uppercase {
        charset.push_str(UPPERCASE);
    }
    if include_numbers {
        charset.push_str(NUMBERS);
    }
    if include_special {
        charset.push_str(SPECIAL_CHARACTERS);
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
    
    password
}

// Function to check the strength of the password
fn check_password_strength(password: &str) -> &str {
    let length = password.len();
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_numbers = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| SPECIAL_CHARACTERS.contains(c));

    if length >= 12 && has_uppercase && has_lowercase && has_numbers && has_special {
        "Strong"
    } else if length >= 8 && (has_uppercase || has_numbers || has_special) {
        "Medium"
    } else {
        "Weak"
    }
}

// Function to check randomness by counting unique characters
fn check_password_randomness(password: &str) -> f64 {
    let unique_chars: HashSet<char> = password.chars().collect();
    unique_chars.len() as f64 / password.len() as f64
}

// Function to get user input as usize
fn get_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Function to get a yes/no input from user
fn get_yes_no_input(prompt: &str) -> bool {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}

fn main() {
    let include_uppercase = true;
    let include_numbers = true;
    let include_special = true;

    loop {
        // Get user input for password length
        let password_length = get_input("Enter the desired password length:");

        // Generate password
        let password = generate_password(password_length, include_uppercase, include_numbers, include_special);
        println!("Generated password: {}", password);

        // Check password strength
        let strength = check_password_strength(&password);
        println!("Password strength: {}", strength);

        // Check password randomness
        let randomness = check_password_randomness(&password);
        println!("Password randomness: {:.2}%", randomness * 100.0);

        // Ask if the user wants to generate a new password
        if !get_yes_no_input("Do you want to generate a new password? (yes/no)") {
            break;
        }
    }
}
