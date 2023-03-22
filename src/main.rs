extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is: {}", secret_number);
	let mut wins: u32 = 0;
	loop {
            println!("Please input yout guess.");
            let mut guess = String::new();
            io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	    };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => {
                    println!("You win!");
		    wins = wins + 1;
		    println!("You've won {} times", wins);
			break;
		}
            }
	}
	println!("Want another round? [y/n] ");
	
        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");

	if(ans == "n"){
	    break;
	};
    }
}
