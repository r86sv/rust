#![allow(unused)]
/// Язык программирования Rust
/// 7 Управление растущими проектами с помощью пакетов, крейтов и модулей
/// 

mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}