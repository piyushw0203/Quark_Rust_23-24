#![allow(unused)]
#![allow(non_snake_case)]
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Choose difficulty\n
    1. First 25% of choices(1-25 in numbers and first 6 characters).\n
    2. First 50% of choices(1-50 in numbers and first 13 characters).\n
    3. All choices");
    let mut n = String::new();
    io::stdin().read_line(&mut n)
    .expect("Could not take input");
    let mut choice: u8 = n.trim().parse().expect("Enter integer value");
    let (secretI, secretC, scs): (u8, u8, char) = match choice {
        1 => {
            let k: u8 = rand::thread_rng().gen_range(1..=25);
            let c: u8 = rand::thread_rng().gen_range(65..=70);
            let c1: char = c as char;
            (k, c, c1)
        },
        2 => {
            let k: u8 = rand::thread_rng().gen_range(1..=50);
            let c: u8 = rand::thread_rng().gen_range(65..=77);
            let c1: char = c as char;
            (k, c, c1)
        },
        3 => {
            let k: u8 = rand::thread_rng().gen_range(1..=100);
            let c: u8 = rand::thread_rng().gen_range(65..=90);
            let c1: char = c as char;
            (k, c, c1)
        },
        _ => {
            let k: u8 = rand::thread_rng().gen_range(1..=100);
            let c: u8 = rand::thread_rng().gen_range(65..=90);
            let c1: char = c as char;
            (k, c, c1)
        },
    };
    println!("{secretI}");
    println!("{scs}");
    let mut flagi = false;
    let mut flagc = false;
    loop {
        if flagi == false {
            println!("Input your guess your number guess");
            let mut guess1 = String::new();
            io::stdin().read_line(&mut guess1)
            .expect("Failed to read line");
            let guess1: u8 = match guess1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
            match guess1.cmp(&secretI) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too large"),
                Ordering::Equal => {
                    println!("Well Done");
                    flagi = true;
                }
            }
        }
        if flagc == false {
            println!("Input your character guess");
            let mut guess2: String = String::new();
            io::stdin().read_line(&mut guess2)
            .expect("Failed to read");
            let guess2: u8 = guess2.bytes().nth(0)
            .expect("No byte read");
            match guess2.cmp(&secretC) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too large"),
                Ordering::Equal => {
                    println!("Well Done");
                    flagc = true;
                }
            }
        }
        if (flagc == true && flagi == true) {
            println!("You win");
            break;
        }

        println!("Status");
        if flagi == true {
            println!("Integer geuss done");
        }
        else {
            println!("Integer guess not done");
        }
        if flagc == true {
            println!("Character geuss done");
        }
        else {
            println!("Character guess not done");
        }
    }
}