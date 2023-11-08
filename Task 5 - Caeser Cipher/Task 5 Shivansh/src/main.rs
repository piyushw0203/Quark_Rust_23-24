#![allow(unused)]
#![allow(non_snake_case)]

use std::io;
use std::cmp::Ordering;


enum Cipher {
    Space(char),
    Number(u8),
    Upper(u8),
    Lower(u8),
    None
}

impl Cipher {
    fn new(element: char) -> Cipher {
        let n: u8 = element as u8;
        if element == ' ' {
            Cipher::Space(element)
        }
        else if n >= 48 && n <= 57 {
            Cipher::Number(n)
        }
        else if n >= 65 && n <= 90 {
            Cipher::Upper(n)
        }
        else if n >= 97 && n <= 122 {
            Cipher::Lower(n)
        }
        else {
            Cipher::None
        }
    }
}

fn cipher(ENUM: Cipher, Number: u8) {
    let mut num: u8 = 0;
    match ENUM {
        Cipher::Space(space) => {
            print!("{space}");
        },
        Cipher::Number(n) => {
            num = (n as char).to_string().trim().parse().expect("Error parsing");
            if num < 7 {
                num = num + 3;
                print!("{num}");
            }
            else {
                num = 9 - num;
                num = 0 + num;
                print!("{num}");
            }
        },
        Cipher::Upper(upper) => {
            if upper < (90 - Number + 1) {
                num = upper + Number;
                let c = num as char;
                print!("{c}");
            }
            else {
                num = 90 - upper;
                num = 64 + Number - num;
                let c = num as char;
                print!("{c}");
            }
        },
        Cipher::Lower(lower) => {
            if lower < (122 - Number + 1) {
                num = lower + Number;
                let c = num as char;
                print!("{c}");
            }
            else {
                num = 122 - lower;
                num = 96 + Number - num;
                let c = num as char;
                print!("{c}");
            }
        },
        _ => {
            println!("Wrong input");
        },
    }
}

fn main() {
    println!("Enter text to be ciphered");
    let mut S = String::new();
    io::stdin().read_line(&mut S);
    println!("Enter shift value");
    let mut x = String::new();
    io::stdin().read_line(&mut x);
    let num = x.trim().parse().expect("Enter valid number");
    if !(num <= 25) {
        return
    }
    for (i, c) in S.trim().chars().enumerate() {
        let character: Cipher = Cipher::new(c);
        cipher(character, num);
    }
}
