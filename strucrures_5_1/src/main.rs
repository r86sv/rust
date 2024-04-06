#![allow(unused)]
/// Язык программирования Rust
/// 5 Использование структур для структурирования связанных данных
/// 5.1 Определение и инициализация структур

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // Создаем нового пользователя с начальными значениями
    let mut user1 = User {
        active: true, // активный
        username: String::from("John"), // имя пользователя
        email: String::from("emai@mail.com"), // электронная почта
        sign_in_count: 0, // количество входов в систему
    };

    // Обновляем адрес электронной почты пользователя
    user1.email = String::from("email_1@mail.com");

    // Создаем второго пользователя с помощью функции `build_user`
    let user2 = build_user(
        String::from("third_email@mail.com"), // электронная почта
        String::from("Username_3"), // имя пользователя
    );

    // Создаем еще одного пользователя, копируя данные из `user1` и обновляя электронную почту
    let user2 = User {
        active: user1.active, // активный
        username: user1.username, // имя пользователя
        email: String::from("another@example.com"), // новая электронная почта
        sign_in_count: user1.sign_in_count, // количество входов в систему
    };
    
    // Создаем третьего пользователя, копируя все данные из `user2`, кроме электронной почты
    let user3 = User {
        email: String::from("another@example.com"), // новая электронная почта
        ..user2
    };

    let black = Color(0, 0, 0); // Создаем экземпляр структуры `Color` с черным цветом
    let origin = Point(0, 0, 0); // Создаем экземпляр структуры `Point`, представляющий начало координат

    let subject = AlwaysEqual; // Создаем экземпляр структуры `AlwaysEqual`
}

/// Создает новый экземпляр `User`.
///
/// Эта функция принимает два аргумента: электронную почту и имя пользователя,
/// и возвращает новый экземпляр `User` с заданными значениями. Пользователь
/// автоматически устанавливается как активный (`active: true`) и счетчик входов
/// устанавливается в 1.
///
/// # Arguments
///
/// * `email` - Строка `String`, содержащая электронную почту пользователя.
/// * `username` - Строка `String`, содержащая имя пользователя.
///
/// # Returns
///
/// Возвращает новый экземпляр `User` с заданными электронной почтой и именем пользователя,
/// установленным в активное состояние и счетчиком входов, равным 1.
fn build_user(email: String, usermame: String) -> User {
    User {
        active: true,
        username: usermame,
        email: email,
        sign_in_count: 1,
    }
    
}

/// Создает новый экземпляр `User` с использованием синтаксиса полей структуры.
///
/// Эта функция демонстрирует использование синтаксиса полей структуры, где имена параметров
/// функции совпадают с именами полей структуры, позволяя опустить указание значений полей
/// структуры, если они имеют такие же имена, как и параметры функции.
///
/// # Arguments
///
/// * `email` - Строка `String`, содержащая электронную почту пользователя.
/// * `username` - Строка `String`, содержащая имя пользователя.
///
/// # Returns
///
/// Возвращает новый экземпляр `User` с заданными электронной почтой и именем пользователя,
/// установленным в активное состояние и счетчиком входов, равным 1.
fn build_user_1(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
