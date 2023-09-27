use std::io;

//passing vector by reference
//Option<usize> specifies that we will be returning the index

fn selection_sort(arr: &mut Vec<i32>) {
    // Find the ith smallest element and place it at i;
    for i in 0..arr.len() - 1 {
        let mut s_idx: usize = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[s_idx] {
                s_idx = j;
            }
        }
        // Swap arr[i] with arr[s_idx] at the end of the inner loop
        arr.swap(i, s_idx);
    }
}

fn find_index(arr: &Vec<i32>, target: i32, start: usize, end: usize) -> Option<usize> {
    if start <= end {
        let mid = (start + end) / 2;
        println!("mid value:{}", mid);
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            return find_index(arr, target, mid + 1, end);
        } else {
            return find_index(arr, target, start, mid - 1);
        }
    }
    None // Return None if the target is not found in the current range
}

fn binary_search(arr: &Vec<i32>, target: i32) {
    let index: Option<usize> = find_index(arr, target, 0, arr.capacity() - 1);
    match index {
        Some(idx) => {
            println!("Number found at index:{}", idx);
        }
        None => {
            println!("Did not find the target element in the array")
        }
    }
}
fn main() {
    let mut inputval = String::new();
    println!("Enter the size of the array ");
    io::stdin()
        .read_line(&mut inputval)
        .expect("Failed to get inputval");

    let arr_size: Result<usize, _> = inputval.trim().parse();
    //u32 is the type that represents the successful result
    // _ is like ignore
    //trim will remove any leading or traling whitespaces
    //parse will typecast the string into expected type, which is u32 here

    let mut my_vec: Vec<i32> = Vec::new();
    //match is similar to switch
    match arr_size {
        Ok(size) => {
            println!("Creating an array of size : {}", size);
            my_vec = Vec::with_capacity(size);
            // You can now use 'num' as a number in your program
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
        }
    }

    //now i have the size of the array, now creating array
    for i in 0..=my_vec.capacity() - 1 {
        let mut curr_element = String::new();
        println!("Enter the {i} element in the array");

        io::stdin()
            .read_line(&mut curr_element)
            .expect("Failed to read input");

        let curr_num: Result<i32, _> = curr_element.trim().parse(); //typecasting the string into 32 bit int
        match curr_num {
            Ok(num) => {
                println!("Element {num} entered successfully");
                my_vec.push(num);
            }
            Err(_) => {
                println!("Enter valid number")
            }
        }
    }

    //printing elements of the array
    for item in &my_vec {
        print!("{} ", item);
    }

    selection_sort(&mut my_vec);
    println!("\n\nprinting the sorted array using selection sort:");
    for i in &my_vec {
        print!("{} ", i);
    }

    println!();
    println!();

    println!("Enter the element you want to search in the array:");

    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("failed to read input");

    //now typecasting the string
    let target_num: Result<i32, _> = target.trim().parse();

    let mut _element_index: Option<usize> = None; // Use Option<usize> to match the return type

    match target_num {
        Ok(num) => {
            binary_search(&my_vec, num);
        }
        Err(_) => {
            println!("Enter a valid integer.");
        }
    }
}
