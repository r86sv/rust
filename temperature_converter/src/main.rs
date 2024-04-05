use std::io;

// Т(°C) = (Т(°F) - 32) × 5/9
// Т(°F) = Т(°C) × 9/5 + 32

fn main() {
    loop {
        clear_terminal();

        let result: u8 = loop {
            let mut buf = String::new();

            println!("Сделайте выбор конвертации температуры:");
            println!("1 - Цельсия =>> Фаренгейты");
            println!("2 - Фаренгейты =>> Цельсия");

            io::stdin()
                .read_line(&mut buf)
                .expect("Не удалось прочитать строку!");

            let buf: u8 = match buf.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            clear_terminal();

            if buf == 1 {
                println!("Конвертация температуры (Цельсия =>> Фаренгейты)");
                println!("Введите температуру в градусах Цельсия:");
                break buf;
            } else if buf == 2 {
                println!("Конвертация температуры (Фаренгейты =>> Цельсия)");
                println!("Введите температуру в градусах Фаренгейта:");
                break buf;
            } else {
                println!("Ваш быбор: {buf}");
                continue;
            }
        };

        let mut temperature = String::new();
        let convertation: i32;

        io::stdin()
            .read_line(&mut temperature)
            .expect("Не удалось прочитать строку!");

        let temperature: i32 = temperature
            .trim()
            .parse()
            .expect("Не удалось преобразовать строку в число!");

        clear_terminal();

        if result == 1 {
            convertation = temperature * 9 / 5 + 32;
            println!("Температура {temperature} по Цеьсию равна {convertation} по Фаренгейту.");
        } else {
            convertation = (temperature - 32) * 5 / 9;
            println!("Температура {temperature} по Фаренгейту равна {convertation} по Цельсию.");
        }

        println!("Чтобы выйти из программы введите 0 и Enter");

        let mut buf = String::new();

        io::stdin()
            .read_line(&mut buf)
            .expect("Не удалось прочитать строку!");

        let buf: u8 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        clear_terminal();

        if buf == 0 {
            break;
        };
    }
}

fn clear_terminal() {
    console::Term::stdout()
        .clear_screen()
        .expect("Не удалось очистить консоль");
}
