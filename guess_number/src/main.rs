// import io for output
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    //cargo doc --open to find how to use the crate
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!(“The secret number is: {secret_number}“);
   loop{
        println!("Please input your guess.");
        //create variable
        let mut guess = String::new();
        //call io module to handle user input
        io::stdin()
            // & inducate it is a reference
            .read_line(&mut guess)
            // same as io::stdin().read_line(&mut guess).expect(“Failed to read line”);
            //if the “result” of read_line is “Err” it will trigger .expect
            .expect("Failed to read line");
        // guess : u32, annotate the variable’s type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //instead of crashing program with non-number guess, prompt user to enter new value
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        // compare guess with secret number
        match guess.cmp(&secret_number) {
            // ordiering type have 3 variant, less, greater equal
            // guess.cmp(&secret_number) return Ordering::Greater, which match next next line
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }
}