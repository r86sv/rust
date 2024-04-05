fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelvth",
    ];

    let lines = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese-a-laying",
        "Seven swans-a-swimming!",
        "Eight maids-a-milking!",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve lords-a-leaping",
    ];

    clear_terminal();

    for i in 0..12 {
        verce_top(days[i]);
        verce_midle(i, lines);
        verce_buttom(days[i], lines);
        println!("");
    }
}

fn verce_top(day: &str) {
    println!("On the {day} day of Christmas");
    println!("My true love gave to me");
}

fn verce_midle(mut itr: usize, line: [&str; 12]) {
    while itr > 0 {
        println!("{}", line[itr]);
        itr -= 1;
    }
}

fn verce_buttom(day: &str, line: [&str; 12]) {
    if day == "first" {
        println!("A {}", line[0]);
    } else {
        println!("And {}", line[0]);
    }
}

fn clear_terminal() {
    console::Term::stdout()
        .clear_screen()
        .expect("Не удалось очистить консоль");
}
