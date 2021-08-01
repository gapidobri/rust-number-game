use rand::Rng;
use std::io::{stdin, stdout, Write};

fn main() {
    clear_scr();
    question(0);
}

fn get_rnd_number() -> u8 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..20);
    return number;
}

fn get_user_input() -> u8 {
    let mut s = String::new();
    print!("Enter your answer: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("You didn't entered a string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    let number = s.parse::<u8>().unwrap();
    return number;
}

fn is_equal(num1: u8, num2: u8, sum: u8) -> bool {
    return num1 + num2 == sum;
}

fn clear_scr() {
    print!("{}[2J", 27 as char);
}

fn question(score: u8) {
    let number1 = get_rnd_number();
    let number2 = get_rnd_number();
    println!("What is {} + {}?", number1, number2);
    let sum = get_user_input();
    clear_scr();
    if is_equal(number1, number2, sum) {
        println!("Correct! - Score: {}\n", score + 1);
        question(score + 1);
    } else {
        println!("Incorrect!");
        println!("Your score is {}", score);
    }
}
