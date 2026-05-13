use crate::utilities; // Используем путь от корня крейта

pub fn start_game() {
    let mut guess_counter: u8 = 0;
    guess_the_riddle(&mut guess_counter);
}

fn guess_the_riddle(c: &mut u8) {

    println!("Отгадайте загадку: Что всегда идет, но никогда не приходит?");
    if utilities::read_input_string().to_lowercase() == "время" {
        println!("Правильно! Это время.");
    } else {
        *c += 1;
        if *c >= 3 {
            println!("Вы исчерпали все попытки ({}). Правильный ответ: время.", c);
            return;
        } else {
            println!("Неправильно. Осталось попыток: {}.", 3 - *c);
            println!("Попробовать еще раз? (да/нет)");
            if utilities::read_input_string().to_lowercase() == "да" {
                guess_the_riddle(c); // Передаем обновленный счетчик
            } else {
                println!("Спасибо за игру!");   
            }
        }
    }   
}
