use std::io;

const C: f32 = 32.0;

fn c_to_f(celsius_temp: f32) -> f32 {
    (celsius_temp * (9.0 / 5.0)) + C
}

fn f_to_c(fahrenheit_temp: f32) -> f32 {
    (fahrenheit_temp - C) * (5.0 / 9.0)
}

fn convert(temperature: f32, choice: u8) -> Option<(f32, &'static str)> {
    match choice {
        1 => Some((c_to_f(temperature), "°F")), // Цельсий -> Фаренгейт
        2 => Some((f_to_c(temperature), "°C")), // Фаренгейт -> Цельсий
        _ => None,
    }
}

pub fn temp_converter() {

    println!("Temperature converter. \n (1) C to F \n (2) F to C");

    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = match user_choice.trim().parse::<u8>() {
        Ok(1) | Ok(2) => {
            user_choice.trim().parse::<u8>().unwrap()  
        },
        _ => {
            println!("Invalid choice. Please select 1 or 2.");
            return;
        }
    };

    println!("Enter temperature:");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature).unwrap();
        
    let temperature = temperature
        .trim()
        .parse::<f32>()
        .expect("Please type a number");

    match convert(temperature, n_choice) {
        Some((value, unit)) => println!("The result of the conversion is: {} {}", value, unit),
        None => println!("Invalid choice. Please select 1 or 2."),
    };

}

