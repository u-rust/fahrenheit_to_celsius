use std::io;

fn main() {
    println!("Введите фаренгейты: ");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).unwrap();
    let fahrenheit:f32 = fahrenheit.trim().parse().expect("Ошибка парсинга");

    let celsius: f32 = fahrenheit_to_celsius(fahrenheit);
    println!("{} градусов Фаренгейта = {} градусов Цельсия", fahrenheit, celsius);
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
