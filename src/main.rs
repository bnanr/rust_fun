use rand::Rng;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::process::{self};

#[derive(Debug)]
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

fn main() {
    loop {
        let choice = loop {
            println!("What would you like to do? (Enter a number)");
            println!("1. Number guessing game\n2. Calculator\n3. Exit");
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
            3 => exit(),
            _ => println!("No choice"),
        }
    }
}

fn number_game() {
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
        println!("What would you like to do?\n1. Add\n2. Subtract\n3. Multiply\n4. Divide\n5. Exit calculator");
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

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mult(a: i32, b: i32) -> i32 {
    a * b
}

fn divi(a: i32, b: i32) {
    if b != 0 {
        let a_mul = (a as u128) * 1000;
        let b = b as u128;
        let div = a_mul / b;

        let frac = div % 1000;
        let rest = div / 1000;
        println!("Result: {}.{:#03}", rest, frac);
    } else {
        println!("Cannot divide by 0!");
    }
}

fn read_line_input() -> Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_owned())
}

fn exit() {
    println!("Exiting application.");
    process::exit(0);
}
