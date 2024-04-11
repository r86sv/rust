#![allow(unused)]
/// Язык программирования Rust
/// 7 Управление растущими проектами с помощью пакетов, крейтов и модулей
/// 7.2 Определение модулей для контроля видимости и закрытости

/// crate
///  └── front_of_house
///      ├── hosting
///      │   ├── add_to_waitlist
///      │   └── seat_at_table
///      └── serving
///          ├── take_order
///          ├── serve_order
///          └── take_payment

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/// Посещение ресторана.
pub fn eat_at_restoutant() {
    // Абсолютный путь
    crate::front_of_house::hosting::add_to_waitlist();
    // Относительный путь
    front_of_house::hosting::add_to_waitlist();
}

/// Доставляет заказ клиенту.
///
/// Эта функция имитирует процесс доставки заказа клиенту в сценарии ресторана.
/// Это функция-заглушка без параметров и возвращаемых значений, представляющая последний шаг в процессе обработки заказа.
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod back_of_house1 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

/// Прием завтрака в ресторане.
///
/// Эта функция демонстрирует, как создать экземпляр `Breakfast` с помощью конструктора `summer`,
/// который позволяет указать желаемый тип тоста. Затем показывается, как изменить публичное поле
/// структуры `Breakfast`. Пример также иллюстрирует, что прямой доступ к приватным полям структуры
/// вне модуля, где она определена, невозможен.
///
/// # Примеры
///
/// ```
/// let mut meal = back_of_house1::Breakfast::summer("Rye");
/// meal.toast = String::from("Wheat");
/// println!("Я бы хотел тост из {} пожалуйста", meal.toast);
/// // Попытка доступа или изменения `meal.seasonal_fruit` приведет к ошибке компиляции,
/// // поскольку `seasonal_fruit` является приватным полем.
/// ```
pub fn eat_at_restaurant() {
    // Заказываем завтрак летнего ассортимента с ржаным тостом
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    // Меняем свое мнение относительно того, какой хлеб мы хотели бы
    meal.toast = String::from("Wheat");
    println!("Я бы хотел тост из {} пожалуйста", meal.toast);

    // Следующая строка не скомпилируется, если мы ее раскомментируем; нам не разрешено
    // видеть или изменять сезонный фрукт, который идет в комплекте с блюдом
    // meal.seasonal_fruit = String::from("черника");
}

mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

/// Заказ аппетитов в ресторане.
///
/// Эта функция демонстрирует создание заказов для аппетитов: супа и салата.
/// Она создает два экземпляра перечисления `Appetizer`, каждый из которых представляет
/// различные варианты аппетитов, доступных в ресторане. Функция не принимает параметров
/// и не возвращает значений, служа примером использования перечислений в Rust.
pub fn eat_at_restaurant1() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}

use crate::front_of_house::hosting;
/// Добавляет клиента в список ожидания.
///
/// Эта функция вызывает функцию `add_to_waitlist` из модуля `hosting`, которая находится в модуле `front_of_house`.
/// Она не принимает параметров и не возвращает значений. Используется для имитации добавления клиента в список ожидания ресторана.
pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}
