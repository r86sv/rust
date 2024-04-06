/// Язык программирования Rust
/// 4.0 Понимание владения
/// 4.1 Что такое "владение"?

fn main() {
    // Инициализация строкового среза со значением "Hello".
    let s = "Привет";
    // Инициализация строкового среза со значением " world!".
    let s1 = " Rust!";

    // Преобразование строкового среза в объект `String` для возможности модификации.
    let s = String::from(s);

    // Объединение двух строк и получение новой строки.
    let (s, len) = string_slice_to_string(&s, s1);

    // Вывод результата объединения строк.
    println!("Строка: {} длиной {} симвлов.", s, len);
}

/// Объединяет строку типа `String` и строковый срез `&str`, возвращая новую строку `String` и её длину.
///
/// Эта функция принимает владеющую строку (`String`) и строковый срез (`&str`),
/// объединяет их в новую строку `String`, а затем возвращает эту новую строку вместе с её длиной.
///
/// # Параметры
///
/// * `s` - Ссылка на строку типа `String`, к которой будет добавлен строковый срез. Эта строка не изменяется.
/// * `s1` - Строковый срез `&str`, который будет добавлен к строке `s`.
///
/// # Возвращаемое значение
///
/// Возвращает кортеж, содержащий новую строку `String`, полученную в результате объединения `s` и `s1`,
/// а также длину этой новой строки в виде `usize`.
fn string_slice_to_string(s: &String, s1: &str) -> (String, usize){
    // Создание новой строки из `s` для возможности модификации.
    let mut s = String::from(s);
    // Добавление `s1` к новой строке.
    s.push_str(s1);

    // Вычисление длины новой строки.
    let leight = s.len();

    // Возвращение новой строки и её длины.
    (s, leight)
}
