use rand::{random, thread_rng, Rng};


//Create the secret number
fn set_rand() -> i8{
    let mut x = thread_rng();
    x.gen_range(1..11)
}

//Create the guess function
fn check_guess(guess: i8, secret: i8) -> i8{
    if guess == secret {
        return 0
    }
    else if guess < secret {
        return -1
    }

    return 1
}

fn main() {
    let mut secret = set_rand();

    let mut guess = set_rand();
    let mut guesses = 1;

    while check_guess(guess, secret) != 0 {
        guesses += 1;
        match check_guess(guess, secret) {
            1 => {
                println!("You guessed: {}, too high!", guess);
                guess -= 1;

            }
            -1 => {
                println!("You guessed: {}, too low!", guess);
                guess += 1;
            }
            _ => {}
        }
    }

    println!("You guessed: {}, you found the secret number in {} guesses!", guess, guesses);

}
