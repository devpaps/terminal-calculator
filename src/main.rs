use std::io;

struct Numbers {
    num_one: f64,
    num_two: f64,
}

#[derive(Debug)]
struct History {
    operations: Vec<String>,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

trait HistoryTrait {
    fn add_history(&mut self, operation: String);
    fn print_history(&self);
}

trait Arithmetic {
    fn sum(&mut self, history: &mut History) -> f64;
    fn sub(&mut self, history: &mut History) -> f64;
    fn product(&mut self, history: &mut History) -> f64;
    fn kvot(&mut self, history: &mut History) -> f64;
}

impl HistoryTrait for History {
    fn add_history(&mut self, operation: String) {
        self.operations.push(operation);
    }
    fn print_history(&self) {
        for operation in &self.operations {
            println!("{}", operation);
        }
    }
}

impl Arithmetic for Numbers {
    fn sum(&mut self, history: &mut History) -> f64 {
        let result = self.num_one + self.num_two;
        history.add_history(format!("{} + {} = {}", self.num_one, self.num_two, result));
        result
    }
    fn sub(&mut self, history: &mut History) -> f64 {
        let result = self.num_one - self.num_two;
        history.add_history(format!("{} - {} = {}", self.num_one, self.num_two, result));
        result
    }
    fn product(&mut self, history: &mut History) -> f64 {
        let result = self.num_one * self.num_two;
        history.add_history(format!("{} * {} = {}", self.num_one, self.num_two, result));
        result
    }
    fn kvot(&mut self, history: &mut History) -> f64 {
        if self.num_two == 0.0 {
            println!("\nYou can't divide by zero!");
            return 0.0;
        }
        let result = self.num_one / self.num_two;
        history.add_history(format!("{} / {} = {}", self.num_one, self.num_two, result));
        result
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
        println!("5. Exit");

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
            Ok(5) => {
                println!("\nBye bye!");
                std::process::exit(0);
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

    let mut history = History {
        operations: Vec::new(),
    };

    loop {
        let operation = validate_menu("Choose an operation:");
        let num_one = validate_number("Enter first number");
        let num_two = validate_number("Enter second number");

        let mut numbers = Numbers { num_one, num_two };

        match operation {
            Operation::Add => {
                let result = numbers.sum(&mut history);
                println!(
                    "The sum of {} + {} is {}",
                    numbers.num_one, numbers.num_two, result
                );
            }
            Operation::Subtract => {
                let result = numbers.sub(&mut history);
                println!(
                    "The difference of {} - {} is {}",
                    numbers.num_one, numbers.num_two, result
                );
            }
            Operation::Multiply => {
                let result = numbers.product(&mut history);
                println!(
                    "The product of {} * {} is {}",
                    numbers.num_one, numbers.num_two, result
                );
            }
            Operation::Divide => {
                let result = numbers.kvot(&mut history);
                println!(
                    "The quotient of {} / {} is {}",
                    numbers.num_one, numbers.num_two, result
                );
            }
        }
        println!("\nOperation history:");
        history.print_history();
    }
}
