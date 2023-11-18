use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[allow(dead_code)]
pub fn start_guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number!");

        println!("Please input your guess");

        let mut guess = String::new();

        // let read_result = match io::stdin().read_line(&mut guess) {
        //     Ok(_) => {
        //         println!("OK");
        //         1
        //     }
        //     Err(err) => {
        //         println!("Read not ok with reason: {:?}", err);
        //         0
        //     }
        // };

        io::stdin().read_line(&mut guess).unwrap_or_else(|error| {
            panic!("Failed to read line. Reason: {:?}", error);
        });

        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(input) => input,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low"),
        };
    }
}
