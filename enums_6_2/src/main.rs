#![allow(unused)]
/// Язык программирования Rust
/// 6 Перечисления и сопоставление с образцом
/// 6.2 Управляющая конструкция match

#[derive(Debug)]
enum UsState {
    Алабама,
    Аляска,
    Аризона,
    Арканзас,
    Калифорния,
    // States...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Вызывает функцию `value_in_cents` с монетой четвертак, указывая штат Аляска,
    // и выводит стоимость монеты и сообщение о штате.
    value_in_cents(Coin::Quarter((UsState::Аляска)));

    // Создает `Option<i32>` со значением `Some(5)`.
    let five = Some(5);
    // Вызывает функцию `plas_one` с `Option<i32>` `five`, увеличивая его значение на 1,
    // и сохраняет результат в `six` как `Option<i32>`.
    let six = Some(plas_one(five));
    // Вызывает функцию `plas_one` с `None`, возвращая `None`,
    // и сохраняет результат в переменной `none`.
    let none = plas_one(None);
}

/// Возвращает стоимость монеты в центах.
///
/// Эта функция принимает монету `coin` и возвращает её стоимость в центах.
/// Для монеты типа `Quarter`, также выводится сообщение с указанием штата,
/// который эмитировал монету.
///
/// # Параметры
///
/// * `coin` - Монета, стоимость которой необходимо определить.
///
/// # Возвращает
///
/// Стоимость монеты в центах как `u8`.
///
/// # Примеры
///
/// ```
/// let coin = Coin::Penny;
/// assert_eq!(value_in_cents(coin), 1);
/// ```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Счастливая копейка!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Этот четвертак эмитировал штат {:?}!", state);
            25
        }
    }
}

/// Увеличивает значение, обернутое в `Option<i32>`, на единицу.
///
/// # Параметры
///
/// * `x` - Опциональное значение типа `i32` для увеличения.
///
/// # Возвращает
///
/// Возвращает `Option<i32>`:
/// - `None`, если `x` равно `None`.
/// - `Some(i + 1)`, если `x` равно `Some(i)`.
///
/// # Примеры
///
/// ```
/// let five = Some(5);
/// let six = plas_one(five);
/// assert_eq!(six, Some(6));
///
/// let none = plas_one(None);
/// assert_eq!(none, None);
/// ```
fn plas_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/// Можно использовать такую конструкцию чтобы обработать остальные варианты
/// ```
/// let dice_roll = 9;
///    match dice_roll {
///        3 => add_fancy_hat(), // вариант
///        7 => remove_fancy_hat(), // вариант
///        other => move_player(other), // остальные варианты
///    }
/// 
///     match dice_roll {
///         3 => add_fancy_hat(),
///         7 => remove_fancy_hat(),
///         _ => reroll(), // во всех остальных вариантах выполнится эта функция
///     }
/// 
///     match dice_roll {
///         3 => add_fancy_hat(),
///         7 => remove_fancy_hat(),
///         _ => (), // все остальные варианты игнорируются
///     }
/// ```
fn void() {}
