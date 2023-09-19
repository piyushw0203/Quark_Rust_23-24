use std::io;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;



fn main() -> std::io::Result<()> {

     let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("output.txt")?;

    println!("Welcome to the Guessing Game!!!\n");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut chances_left = 10;
    let mut parsed_guess: Result<i32, _>;
    let mut username = String::new();

    println!("Enter your username");
    io::stdin().read_line(&mut username).expect("Failed to read username");

    let mut win_status :bool = false;



    while chances_left > 0 {
        println!("Enter a random guess");

        // &mut guess means we let read_line function borrow the value pointed by guess
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        // Debugging: Print the value of guess before parsing
        println!("Debug: User input: {}", guess);

        // Typecasting to number
        parsed_guess = guess.trim().parse();
        let mut guessed_number = 0;

        guess.clear();

        // Ensuring whether the number is typecasted successfully
        match parsed_guess {
            Ok(guessed_num) => {
                guessed_number = guessed_num;
            }
            Err(_) => {
                println!("Invalid number");
            }
        }

        if guessed_number == secret {
            println!("Correct Guess");
            win_status = true;
            break;
        } else if guessed_number < secret {
            println!("A bit more!\n\n");
            chances_left -= 1;
            println!("You have {} chances left", chances_left);
        } else {
            println!("A bit less\n\n");
            chances_left -= 1;
            println!("You have {} chances left", chances_left);
        }
    }
    println!("\n\nThe secret number was {}", secret);

     let text_to_append =  username.to_string()  +"won:"+ &win_status.to_string() + "\nattemps:"+&(10 - chances_left).to_string() +"\n\n";

    file.write_all(text_to_append.as_bytes())?;

    //  flushing the buffer to ensure data is written immediately
    file.flush()?;


    Ok(())
}
