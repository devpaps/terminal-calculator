use std::io;

struct Numbers {
    num_one: i32,
    num_two: i32,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Numbers {
    fn sum(&self) -> i32 {
        self.num_one + self.num_two
    }
    fn sub(&self) -> i32 {
        self.num_one - self.num_two
    }
    fn product(&self) -> i32 {
        self.num_one * self.num_two
    }
    fn kvot(&self) -> i32 {
        self.num_one / self.num_two
    }
}

fn validate_number(prompt: &str) -> i32 {
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

        println!("{}", menu_alternatives);

        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");

        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read line");

        match numbers.trim().parse() {
            Ok(1) => return Operation::Add,
            Ok(2) => return Operation::Subtract,
            Ok(3) => return Operation::Multiply,
            Ok(4) => return Operation::Divide,
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
    println!("What do you want to calculate?");

    let operation = validate_menu("Choose a number");
    let num_one = validate_number("Enter number 1");
    let num_two = validate_number("Enter number 2");

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
                "The product of {} - {} is {}",
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
                "The product of {} / {} is {}",
                numbers.num_one,
                numbers.num_two,
                numbers.kvot()
            );
        }
    }
}
