use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        guessing_game();
        if !play_again() {
            println!("Doei!");
            break;
        }
    }
}

fn play_again() -> bool {
    let another_game = user_input("Wil je nog een keer spelen?").trim();

    if another_game == String::from("ja") || another_game == String::from("j") {
        true
    } else {
        false
    }
}

fn user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Ik kon het niet goed lezen");

    answer
}

fn guessing_game() {
    println!("Raad Het Getal!");

    const MAX_NUMBER: u32 = 10;
    let secret_number = rand::thread_rng().gen_range(1, MAX_NUMBER + 1);

    // println!("Het geheime getal is: {}", secret_number);

    loop {
        println!("Vul een getal tussen 1 en {} in.", MAX_NUMBER);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ik kon het niet goed lezen");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Je raadde: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Te laag!"),
            Ordering::Greater => println!("Te hoog!"),
            Ordering::Equal => {
                println!("Je hebt het geraden!!");
                break;
            }
        }
    }
}
