#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;

fn sorting(mut Arr: [i32; 50], size: usize) -> ([i32; 50], u8) {
    let mut i:usize = 1;
    while i < size {
        let mut j = i;
        while j != 0 {
            if Arr[j] >= Arr[j - 1] {
                break;
            }
            if Arr[j] < Arr[j - 1] {
                let temp = Arr[j];
                Arr[j] = Arr[j - 1];
                Arr[j - 1] = temp;
                j = j - 1;
            }
        }
        i = i + 1;
    }
    (Arr, 1)
}

fn newArr() -> ([i32; 50], u8, usize) {
    println!("Enter size of array (max 50)");
    let mut acc: String = String::new();
    io::stdin().read_line(&mut acc)
        .expect("Enter valid value");
    let mut size: usize = match acc.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter valid value for array");
            51
        }
    };
    size = if size > 50 {
        50
    } else {
        size
    };
    let mut Arr: [i32; 50] = [0; 50];
    println!("Enter value of Array");
    let mut i = 0;
    while i < size {
        let mut acc: String = String::new();
        io::stdin().read_line(&mut acc)
            .expect("Enter valid value");
        let number = match acc.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid value for array");
                -1
            }
        };
        Arr[i] = number;
        i = i + 1;
    }
    (Arr, 0, size)
}

fn search(Arr: [i32; 50], size: usize, flag: u8, key: i32) -> ([i32; 50], usize) {
    if flag != 1 {
        println!("Array has not been sorted");
        (Arr, 51)
    }
    else {
        let mut low = 0;
        let mut high = size - 1;
        let mut mid = ((low + high)/2) as usize;
        while high > low {
            if Arr[mid] < key {
                low = mid + 1;
            }
            else if Arr[mid] > key {
                high = mid - 1;
            }
            else {
                return (Arr, mid)
            }
            mid = ((low + high)/2) as usize;
        }
        if high == low {
            if Arr[low] == key {
                return (Arr, low)
            }
        }
        (Arr, 51)
    }
}

fn display(Arr: [i32; 50], size: usize) -> [i32; 50] {
    let mut i = 0;
    while i < size {
        let x = Arr[i];
        print!("{x} ");
        i = i + 1;
    }
    println!();
    Arr
}

fn main() {
    let (mut Arr, mut flag, mut size): ([i32; 50], u8, usize) = newArr();
    loop {
        println!("Enter your choice
        1. Sorting
        2. Searching
        3. New Array");
        let mut s = String::new();
        io::stdin().read_line(&mut s)
            .expect("Enter valid value");
        let choice = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter proper number!!\n By default 1 will be chosen");
                1
            }
        };

        match choice {
            1 => (Arr, flag) = sorting(Arr, size),
            2 => {
                println!("Enter element to find");
                let mut s = String::new();
                io::stdin().read_line(&mut s)
                    .expect("Enter valid value");
                let key: i32 = match s.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a number");
                        continue;
                    }
                };
                let mut ind;
                (Arr, ind) = search(Arr, size, flag, key);
                if ind < 50 {
                    ind = ind + 1;
                    println!("Element is present at position {ind}");
                }
                else {
                    println!("Element not found");
                }
            },
            3 => (Arr, flag, size) = newArr(),
            _ => continue,
        }
        Arr = display(Arr, size);
        println!("Do you want to continue(Y/N)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Enter valid value");
        let ch = input.chars().nth(0);
        match ch {
            Option::Some(c) => {
                if c.eq_ignore_ascii_case(&'y') {
                    continue;
                }
                else if c.eq_ignore_ascii_case(&'n'){
                    break;
                }
            },
            Option::None => continue,
        }
    }
}
