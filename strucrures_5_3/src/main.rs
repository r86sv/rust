#![allow(unused)]
/// Язык программирования Rust
/// 5 Использование структур для структурирования связанных данных
/// 5.3. Синтаксис метода

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/// Каждая структура может иметь несколько impl
/// в котором каждый метод находится в своём собственном блоке impl.
/// 
/// Это эквивалентный код:
/// ```
/// impl Rectangle {
///     fn area(&self) -> u32 {
///         self.width * self.height
///     }
/// }
/// impl Rectangle {
///     fn width(&self) -> bool {
///         self.width > 0
///     }
/// }
/// impl Rectangle {
///     fn can_hold(&self, other: &Rectangle) -> bool {
///         self.width > other.width && self.height > other.height
///     }
/// }
/// impl Rectangle {
///     fn square(size: u32) -> Self {
///         Self {
///             width: size,
///             height: size,
///         }
///     }
/// }
/// ```

fn main() {
    // Создание трех прямоугольников с различными размерами.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Вывод площади первого прямоугольника.
    println!(
        "Площадь прямоугольника равна {} квадратных пикселей.",
        rect1.area()
    );

    // Проверка, имеет ли первый прямоугольник ненулевую ширину, и вывод её значения.
    if rect1.width() {
        println!(
            "Прямоугольник имеет ненулевую ширину, которая равна {}",
            rect1.width
        );
    }

    // Проверка, может ли первый прямоугольник вместить второй и третий прямоугольники.
    println!("Может ли rect1 вместить rect2? {}", rect1.can_hold(&rect2));
    println!("Может ли rect1 вместить rect2? {}", rect1.can_hold(&rect3));

    // Создание квадрата с помощью ассоциированной функции и вывод его площади.
    let sq = Rectangle::square(10);
    println!("Площадь квадлата равна со стороной 10 равна {}", sq.area());
}
