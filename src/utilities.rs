use std::io;
use std::str::FromStr;

pub fn read_input_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать ввод");
    return input.trim().to_string();
}

pub fn read_input_num<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать ввод");
    return input.trim().parse();
}
