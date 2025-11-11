use rand::Rng;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::process::{self};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum TodoError {
    Io(io::Error),
    Parse(ParseIntError),
    NotFound,
    Serialization(serde_json::Error),
    InvalidInput,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::Io(err) => write!(f, "I/O Error: {}", err),
            TodoError::Parse(err) => write!(
                f,
                "Parsing Error: Expected a number, got something else. ({})",
                err
            ),
            TodoError::NotFound => write!(f, "Application Error: Task not found at that index."),
            TodoError::Serialization(err) => write!(
                f,
                "Data Error: Could not read or write data file. ({})",
                err
            ),
            TodoError::InvalidInput => {
                write!(f, "Input Error: Please enter a valid command or value.")
            }
        }
    }
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> Self {
        TodoError::Io(err)
    }
}

impl From<ParseIntError> for TodoError {
    fn from(err: ParseIntError) -> Self {
        TodoError::Parse(err)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> Self {
        TodoError::Serialization(err)
    }
}

pub type Result<T> = std::result::Result<T, TodoError>;

pub fn start() {
    loop {
        let choice = loop {
            println!("What would you like to do? (Enter a number)");
            println!(
                "1. Number guessing game\n2. Calculator\n3. Cool Calculator\n4. Even Cooler Calculator\n5. Temperature Converter\n6. Exit"
            );
            let input = match read_line_input() {
                Ok(s) => s,
                Err(_) => continue, // io error
            };

            match input.parse::<i32>() {
                Ok(p) => break p,
                Err(e) => {
                    eprintln!("{}", TodoError::Parse(e));
                    continue; // invalid
                }
            }
        };

        match choice {
            1 => number_game(),
            2 => calculator(),
            3 => cool_calculator(),
            4 => even_cooler_calc(),
            5 => temp_convert(),
            6 => exit(),
            _ => println!("No choice"),
        }
    }
}

pub fn number_game() {
    let mut rng = rand::thread_rng();
    loop {
        let random_number = rng.gen_range(1..=10);
        println!("Guess a number between 1-10! Type '0' to go back to the menu.");
        let guess = loop {
            let input = match read_line_input() {
                Ok(s) => s,
                Err(_) => continue, // io error
            };

            match input.parse::<i32>() {
                Ok(p) => break p,
                Err(e) => {
                    eprintln!("{}", TodoError::Parse(e));
                    continue; // invalid
                }
            }
        };
        if guess == 0 {
            break;
        }
        if guess == random_number {
            println!("CORRECT! The right number was {}", random_number);
        } else {
            println!(
                "You guessed: {}, but the right number was: {}",
                guess, random_number
            );
        }
    }
}

fn calculator() {
    loop {
        println!(
            "What would you like to do?\n1. Add\n2. Subtract\n3. Multiply\n4. Divide\n5. Exit calculator"
        );
        let command = loop {
            let input = match read_line_input() {
                Ok(s) => s,
                Err(_) => continue, // io error
            };

            match input.parse::<i32>() {
                Ok(p) => break p,
                Err(e) => {
                    eprintln!("{}", TodoError::Parse(e));
                    continue; // invalid
                }
            }
        };
        if command == 5 {
            break;
        }

        println!("Enter the first number.");
        let first_number = loop {
            let input = match read_line_input() {
                Ok(s) => s,
                Err(_) => continue, // io error
            };

            match input.parse::<i32>() {
                Ok(p) => break p,
                Err(e) => {
                    eprintln!("{}", TodoError::Parse(e));
                    continue; // invalid
                }
            }
        };
        println!("Enter the second number.");
        let second_number = loop {
            let input = match read_line_input() {
                Ok(s) => s,
                Err(_) => continue, // io error
            };

            match input.parse::<i32>() {
                Ok(p) => break p,
                Err(e) => {
                    eprintln!("{}", TodoError::Parse(e));
                    continue; // invalid
                }
            }
        };

        match command {
            1 => {
                let res = add(first_number, second_number);
                println!("Result: {}", res);
            }
            2 => {
                let res = sub(first_number, second_number);
                println!("Result: {}", res);
            }
            3 => {
                let res = mult(first_number, second_number);
                println!("Result: {}", res);
            }
            4 => {
                divi(first_number, second_number);
            }
            _ => println!("Invalid command!"),
        }
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mult(a: i32, b: i32) -> i32 {
    a * b
}

fn divi(a: i32, b: i32) {
    if b != 0 {
        let a_mul = (a as u128) * 1000;
        let b = b as u128;
        let div = a_mul / b;

        let frac = div % 1000;
        let rest = div / 1000;
        println!("Result: {} / {} = {}.{:#03}", a, b, rest, frac);
    } else {
        println!("Cannot divide by 0!");
    }
}

fn cool_calculator() {
    loop {
        println!(
            "This is the better calculator. Please enter an expression, like 7*8. Type exit to quit.\nOnly one expression allowed."
        );

        let mut command = match read_line_input() {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Error reading command: {}", e);
                continue;
            }
        };
        if command.to_lowercase() == "exit" {
            break;
        }
        remove_whitespace(&mut command);

        let first_num;
        let second_num;
        if command.contains('+') {
            let parts = command.split('+');
            let collection = parts.collect::<Vec<&str>>();
            first_num = collection[0].parse::<i32>().unwrap();
            second_num = collection[1].parse::<i32>().unwrap();
            println!(
                "Result: {} + {} = {}",
                first_num,
                second_num,
                add(first_num, second_num)
            );
        } else if command.contains('-') {
            let parts = command.split('-');
            let collection = parts.collect::<Vec<&str>>();
            first_num = collection[0].parse::<i32>().unwrap();
            second_num = collection[1].parse::<i32>().unwrap();
            println!(
                "Result: {} - {} = {}",
                first_num,
                second_num,
                sub(first_num, second_num)
            );
        } else if command.contains('*') {
            let parts = command.split('*');
            let collection = parts.collect::<Vec<&str>>();
            first_num = collection[0].parse::<i32>().unwrap();
            second_num = collection[1].parse::<i32>().unwrap();
            println!(
                "Result: {} * {} = {}",
                first_num,
                second_num,
                mult(first_num, second_num)
            );
        } else if command.contains('/') {
            let parts = command.split('/');
            let collection = parts.collect::<Vec<&str>>();
            first_num = collection[0].parse::<i32>().unwrap();
            second_num = collection[1].parse::<i32>().unwrap();
            divi(first_num, second_num);
        } else {
            println!("Invalid expression.");
        }
    }
}

fn even_cooler_calc() {
    println!("This is the EVEN better calculator!");
    loop {
        println!("Please enter an expression, like 7*8+5. Type exit to quit.");

        let mut command = match read_line_input() {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Error reading command: {}", e);
                continue;
            }
        };
        if command.to_lowercase() == "exit" {
            break;
        }
        remove_whitespace(&mut command);

        let tokens = tokenize(command.as_str());
        let mut iter = tokens.iter();
        let result = parse_expr(&mut iter);
        println!("{} = {}", command, result);
    }
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();

    for ch in expr.chars() {
        match ch {
            '0'..='9' | '.' => num_buf.push(ch),
            '+' | '-' | '*' | '/' => {
                if !num_buf.is_empty() {
                    tokens.push(Token::Number(num_buf.parse().unwrap()));
                    num_buf.clear();
                }
                tokens.push(match ch {
                    '+' => Token::Add,
                    '-' => Token::Subtract,
                    '*' => Token::Multiply,
                    '/' => Token::Divide,
                    _ => unreachable!(),
                });
            }
            ' ' => continue,
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    if !num_buf.is_empty() {
        tokens.push(Token::Number(num_buf.parse().unwrap()));
    }

    tokens
}

fn parse_mul_div(tokens: &mut std::slice::Iter<Token>) -> f64 {
    let mut value = parse_number(tokens);
    while let Some(token) = tokens.clone().next() {
        match token {
            Token::Multiply => {
                tokens.next();
                value *= parse_number(tokens);
            }
            Token::Divide => {
                tokens.next();
                value /= parse_number(tokens);
            }
            _ => break,
        }
    }
    value
}

fn parse_number(tokens: &mut std::slice::Iter<Token>) -> f64 {
    if let Some(Token::Number(n)) = tokens.next() {
        *n
    } else {
        panic!("Expected number");
    }
}

fn parse_expr(tokens: &mut std::slice::Iter<Token>) -> f64 {
    let mut value = parse_mul_div(tokens);
    while let Some(token) = tokens.clone().next() {
        match token {
            Token::Add => {
                tokens.next();
                value += parse_mul_div(tokens);
            }
            Token::Subtract => {
                tokens.next();
                value -= parse_mul_div(tokens);
            }
            _ => break,
        }
    }
    value
}

pub fn read_line_input() -> Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_owned())
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn temp_convert() {
    println!("Convert between Fahrenheit, Celcius and Kelvin.");
    loop {
        println!("Please enter an expression, like 76C, 100K or -12F. Type exit to quit.");
        let mut command = match read_line_input() {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Error reading command: {}", e);
                continue;
            }
        };
        if command.to_lowercase() == "exit" {
            break;
        }

        println!("What unit would you like to convert to? Type C, F or K.");
        let selected_unit = match read_line_input() {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Error reading command: {}", e);
                continue;
            }
        };

        if (command.ends_with('F') || command.ends_with('f')) && (command.len() > 1) {
            command.truncate(command.len() - 1);
            if command.parse::<f64>().is_ok() {
                let res = match selected_unit.to_lowercase().as_str() {
                    "c" => f_to_c(command.parse().unwrap()),
                    "k" => f_to_k(command.parse().unwrap()),
                    _ => 0.0,
                };
                println!("{}F is {}{}.", command, res, selected_unit);
            } else {
                println!("Not a number, try again!");
            }
        } else if (command.ends_with('C') || command.ends_with('c')) && (command.len() > 1) {
            command.truncate(command.len() - 1);
            if command.parse::<f64>().is_ok() {
                let res = match selected_unit.to_lowercase().as_str() {
                    "f" => c_to_f(command.parse().unwrap()),
                    "k" => c_to_k(command.parse().unwrap()),
                    _ => 0.0,
                };
                println!("{}C is {}{}.", command, res, selected_unit);
            } else {
                println!("Not a number, try again!");
            }
        } else if (command.ends_with('K') || command.ends_with('k')) && (command.len() > 1) {
            command.truncate(command.len() - 1);
            if command.parse::<f64>().is_ok() {
                let res = match selected_unit.to_lowercase().as_str() {
                    "f" => k_to_f(command.parse().unwrap()),
                    "c" => k_to_c(command.parse().unwrap()),
                    _ => 0.0,
                };
                println!("{}K is {}{}.", command, res, selected_unit);
            } else {
                println!("Not a number, try again!");
            }
        } else {
            println!("Invalid expression, try again!");
        }
    }
}

pub fn c_to_f(c: f32) -> f32 {
    c * (9.0 / 5.0) + 32.0
}

pub fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

pub fn c_to_k(c: f32) -> f32 {
    c + 273.15
}

pub fn f_to_k(f: f32) -> f32 {
    c_to_k(f_to_c(f))
}

pub fn k_to_c(k: f32) -> f32 {
    k - 273.15
}

pub fn k_to_f(k: f32) -> f32 {
    c_to_f(k_to_c(k))
}

fn exit() {
    println!("Exiting application.");
    process::exit(0);
}
