#![allow(unused)]
/// Язык программирования Rust
/// 8 Общие коллекции
/// 8.2 Хранение закодированного текста UTF-8 в строках

fn main() {
    let mut s= String::new();

    let data = "Инициализация контента";
    // через функцию to_string()
    let s = data.to_string();
    // метод также работает непосредственно с литералом
    let s = "Инициализация контента".to_string();
    // через функцию String::from() можно создать строки из других типов данных
    let s = String::from("Инициализация контента");

    //UTF8 это:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {s2}");
    println!("s1: {s1}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Привет");
    let s2 = String::from(" Мир!");
    let s3 = s1 + &s2; // примечание s1 перенесено сюда и больше не может использоваться
    println!("s3: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");

    // Это не сработает
    /*
    fn main() {
        let s1 = String::from("hello");
        let h = s1[0];
    }
    */

    // Тоже неверно. Букав "З" занимает два байта.
    /*
    let hello = "Здравствуйте";
    let answer = &hello[0];
    */

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {s}");

    for c in "Зд".chars() {
        println!("c: {c}");
    };
    
    for c in "Зд".bytes() {
        println!("Byte: {c}");
    };
}
