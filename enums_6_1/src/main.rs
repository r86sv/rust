#![allow(unused)]
/// Язык программирования Rust
/// 6 Перечисления и сопоставление с образцом
/// 6.1 Определение перечисления

enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Через перечисление
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Аналогия через структуры
struct QuitMessage; // единичная структура
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // кортежная структура
struct ChangeColorMessage(i32, i32, i32); // кортежная структура

fn main() {
    // Объявление переменных
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Вызов функции
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    /*
    *    // Вариант чрез создание экземпляров структуры
    *    let home = IpAddr {
    *        kind: IpAddrKind::V4,
    *        address: String::from("127.0.0.1"),
    *    };
    *    let loopback = IpAddr {
    *        kind: IpAddrKind::V6,
    *        address: String::from("::1"),
    *    };
    // Прикрепление данных к каждому варианту перечисления напрямую
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));

    // Преимущество использования перечисления вместо структуры заключается в том,
    // что каждый вариант перечисления может иметь разное количество ассоциированных
    // данных представленных в разных типах.
    *    enum IpAddr {
    *        V4(u8, u8, u8, u8),
    *        V6(String),
    *    }
    *
    *    let home = IpAddr::V4(127, 0, 0, 1);
    *    let loopback = IpAddr::V6(String::from("::1"));
    */
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    /// Выполняет действие, ассоциированное с сообщением.
    ///
    /// Этот метод позволяет обрабатывать различные типы сообщений, определенные в перечислении `Message`.
    /// В зависимости от варианта `Message`, может быть выполнено различное действие.
    impl Message {
        fn call(&self) {
            // тело метода будет определено здесь
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Ошибка: нельзя сложить i8 и Option<i8>
    // let sum = x + y;
}

/// Маршрутизирует данные пакеты на основе типа IP-адреса.
///
/// Эта функция является заглушкой для более сложной логики маршрутизации, которая бы
/// определяла, как обрабатывать данные пакеты, в зависимости от того, являются ли они IPv4 или IPv6.
///
/// # Параметры
///
/// * `ip_kind`: Тип IP-адреса (`IpAddrKind`), который может быть либо IPv4, либо IPv6.
///
/// # Примеры
///
/// ```
/// route(IpAddrKind::V4);
/// route(IpAddrKind::V6);
/// ```
fn route(ip_kind: IpAddrKind) {}
