
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 8;
    let mut guess = 3;
    let mut attempts = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {} is correct!", guess);
            break;
        } else if result == 1 {
            println!("Guess {} is too high.", guess);
        } else {
            println!("Guess {} is too low.", guess);
        }

        guess += 1;
    }

    println!("You guess it in {} tries!", attempts);
}
