use std::io;

struct Numbers {
    num_one: f64,
    num_two: f64,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

trait Arithmetic {
    fn sum(&self) -> f64;
    fn sub(&self) -> f64;
    fn product(&self) -> f64;
    fn kvot(&self) -> f64;
}

impl Arithmetic for Numbers {
    fn sum(&self) -> f64 {
        self.num_one + self.num_two
    }
    fn sub(&self) -> f64 {
        self.num_one - self.num_two
    }
    fn product(&self) -> f64 {
        self.num_one * self.num_two
    }
    fn kvot(&self) -> f64 {
        if self.num_two == 0.0 {
            println!("\nYou can't divide by zero!");
            return 0.0;
        }
        self.num_one / self.num_two
    }
}

fn validate_number(prompt: &str) -> f64 {
    loop {
        let mut num = String::new();

        println!("{}", prompt);

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        match num.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please type a number!");
            }
        }
    }
}

fn validate_menu(menu_alternatives: &str) -> Operation {
    const MAX: i8 = 4;
    const MIN: i8 = 1;
    loop {
        let mut numbers = String::new();

        println!("\n{}", menu_alternatives);

        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");

        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read line");

        match numbers.trim().parse() {
            Ok(1) => {
                println!("\nYou chose to add!\n");
                return Operation::Add;
            }
            Ok(2) => {
                println!("\nYou chose to subtract!\n");
                return Operation::Subtract;
            }
            Ok(3) => {
                println!("\nYou chose to multiply!\n");
                return Operation::Multiply;
            }
            Ok(4) => {
                println!("\nYou chose to divide!\n");
                return Operation::Divide;
            }
            Ok(_) => {
                println!("Please type a number between {} and {}!", MIN, MAX);
            }
            Err(_) => {
                println!("Please type a number!")
            }
        }
    }
}

fn main() {
    println!("{}", "\nWelcome to the calculator 🚀!");

    let operation = validate_menu("Choose an operation:");
    let num_one = validate_number("Enter first number");
    let num_two = validate_number("Enter second number");

    let numbers = Numbers { num_one, num_two };

    match operation {
        Operation::Add => {
            println!(
                "The sum of {} + {} is {}",
                numbers.num_one,
                numbers.num_two,
                numbers.sum()
            );
        }
        Operation::Subtract => {
            println!(
                "The differens of {} - {} is {}",
                numbers.num_one,
                numbers.num_two,
                numbers.sub()
            );
        }
        Operation::Multiply => {
            println!(
                "The product of {} * {} is {}",
                numbers.num_one,
                numbers.num_two,
                numbers.product()
            );
        }
        Operation::Divide => {
            println!(
                "\nThe kvot of {} / {} is {}",
                numbers.num_one,
                numbers.num_two,
                numbers.kvot()
            );
        }
    }
}
