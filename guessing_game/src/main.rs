use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Сектетный номер: {secret_number}");

    loop {
        println!("Пожалуйста, введите вашу догадку.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать строку.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваша догадка {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Больше!"),
            Ordering::Greater => println!("Меньше!"),
            Ordering::Equal => {
                println!("Угадал!");
                break;
            }
        }
    }
}
