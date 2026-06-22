#![warn(clippy::all, clippy::pedantic)]

use std::io;
use std::str::FromStr;

// Чтение ввода и преобразование в тип T
pub fn read_input_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать ввод");
    return input.trim().to_string();
}

// Чтение ввода и преобразование в тип T
pub fn read_input_num<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать ввод");
    return input.trim().parse();
}

// Определение количества знаков после запятой для числа с плавающей точкой
pub fn def_decimal_precision(num: f64) -> usize {
    let s = num.to_string();
    if let Some(pos) = s.find('.') {
        s[pos + 1..].len()
    } else {
        0
    }
}

// Преобразование строки в f64
pub fn str_to_num(str: &str) -> f64 {
    str.trim().parse::<f64>().expect("Please type a number!")
}

