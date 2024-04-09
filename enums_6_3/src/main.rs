#![allow(unused)]
/// Язык программирования Rust
/// 6 Перечисления и сопоставление с образцом
/// 6.3 Компактное управление потоком выполнения с if let

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Используем `match` для обработки значения `Option`
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Максимум настроен на {}", max),
        _ => (),
    }

    // Используем `if let` для упрощенной обработки значения `Option`
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Максимум настроен на {}", max);
    }

    // Используем `match` для обработки перечисления `Coin`
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("Четвертак штата {:?}!", state),
        _ => count += 1,
    }

    // Используем `if let` и `else` для упрощенной обработки перечисления `Coin`
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("Четвертак штата {:?}!", state);
    } else {
        count += 1;
    }
}
