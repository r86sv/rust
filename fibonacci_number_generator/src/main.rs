// Генератор n-числа Фибоначи
use std::io;

fn main() {
    let mut fib1: u64 = 0;
    let mut fib2: u64 = 1;

    let mut fibonacci: u64 = 0;

    loop {
        clear_terminal();
        let mut n = String::new();
        let mut i: u32 = 1;

        println!("Это генератор n-числа Фибоначи!");
        println!("Ввведите число:");

        io::stdin()
            .read_line(&mut n)
            .expect("Не удалось ввести строку");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        clear_terminal();

        if n == 0 {
            fibonacci = fib1;
        } else if n == 1 {
            fibonacci = fib2;
        } else {
            while i < n {
                fibonacci = fib1 + fib2;
                fib1 = fib2;
                fib2 = fibonacci;

                i += 1;
            }
        }

        println!("Число Фибоначи: {fibonacci}");
        break;
    }
}

fn clear_terminal() {
    console::Term::stdout()
        .clear_screen()
        .expect("Не удалось очистить консоль");
}
