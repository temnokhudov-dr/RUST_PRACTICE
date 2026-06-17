use std::io;
use crate::utilities;


pub fn run_calculator() {
    let mut history: Vec<String> = Vec::new();
    let mut num1_input: bool = true;
    let mut operator_input: bool = false;
    let mut num1: f64 = 0.0;
    let mut num2: f64 = 0.0;
    let mut operator: String = String::new();

    println!("=== Калькулдятор ===");
    println!("Поддерживаемые операции: +, -, *, /, ^");
    println!("Команды: history, clear, exit, return");
    println!();

    loop {
        if num1_input {
            print!("> num1: ");
        } else if operator_input {
            print!("> operator: ");
        } else {
            print!("> num2: ");
        }
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => {
                println!("Выход из калькулятора");
                break;
            }
            "history" => {
                if history.is_empty() {
                    println!("История пуста");
                } else {
                    for (i, entry) in history.iter().enumerate() {
                        println!("№{}: {}", i + 1, entry);
                    }
                }
            }
            "clear" => {
                history.clear();
                println!("История очищена");
            }
            "return" => {
                num1_input = true;
                operator_input = false;
                println!("Возврат к начальному вводу выражений");
            }
            _ => {
                if num1_input {
                    if let Ok(parsed_num) = input.parse::<f64>() {
                        num1 = parsed_num;
                        num1_input = false;
                        operator_input = true;
                    } else {
                        println!("Ошибка: '{}' не является числом", input);
                    }
                } else if operator_input {
                    // Проверяем, что введён корректный оператор
                    if ["+", "-", "*", "/", "^"].contains(&input) {
                        operator = input.to_string();
                        operator_input = false;
                    } else {
                        println!("Ошибка: '{}' не является допустимым оператором", input);
                        println!("Допустимые операторы: +, -, *, /, ^");
                    }
                } else {
                    if let Ok(parsed_num) = input.parse::<f64>() {
                        num2 = parsed_num;
                        num1_input = true;
                        operator_input = false;

                        // Собираем выражение
                        let expresstion= format!("{} {} {}", num1, operator, num2);
                        // Парсим и вычисляем выражение 
                        match calculate(&expresstion) {
                            Ok(result) => {
                                // Сохраняем в историю 
                                let entry = format!("{} = {}", expresstion, result);
                                history.push(entry);
                                println!("Результат: {}", result);
                            }
                            Err(e) => {
                                println!("Ошибка: {}", e);
                            }
                        }

                        println!();

                    } else {
                        println!("Ошибка: '{}' не является числом", input);
                    }
                }
            }
        }

    }

}


pub fn calculate(expression: &str) -> Result<f64, String>{
    // Разбиваем выражение на части 
    let parts: Vec<&str> = expression.split_whitespace().collect();

    // Проверяем формат 
    if parts.len() != 3 {
        return Err("Неверный формат. Используйте: число-оператор-число".to_string());
    }

    // Парсим числа 
    let num1 = parts[0].parse::<f64>()
        .map_err(|_| format!("'{}' не является числом", parts[0]))?;
    let operator = parts[1];
    let num2 = parts[2].parse::<f64>()
        .map_err(|_| format!("'{}' не является числом", parts[2]))?;

    // Вычисляем результат 
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Деление на ноль".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        "^" => Ok(num1.powf(num2)),
        _ => Err(format!("Неизвестный оператор '{}'. Используйте: +, -, *, /, ^", operator)),
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    // ============ ТЕСТЫ ДЛЯ CALCULATE ============

    mod calculate_tests {
        use super::*;

        // === Базовые операции ===

        #[test]
        fn test_add() {
            assert_eq!(calculate("2 + 3").unwrap(), 5.0);
        }

        #[test]
        fn test_subtract() {
            assert_eq!(calculate("10 - 4").unwrap(), 6.0);
        }

        #[test]
        fn test_multiply() {
            assert_eq!(calculate("3 * 4").unwrap(), 12.0);
        }

        #[test]
        fn test_divide() {
            assert_eq!(calculate("10 / 2").unwrap(), 5.0);
        }

        #[test]
        fn test_power() {
            assert_eq!(calculate("2 ^ 3").unwrap(), 8.0);
        }

        // === Числа с плавающей точкой ===

        #[test]
        fn test_float_addition() {
            assert_eq!(calculate("2.5 + 3.7").unwrap(), 6.2);
        }

        #[test]
        fn test_float_multiplication() {
            assert_eq!(calculate("2.5 * 4.0").unwrap(), 10.0);
        }

        #[test]
        fn test_negative_numbers() {
            assert_eq!(calculate("-5 + 3").unwrap(), -2.0);
        }

        // === Граничные случаи ===

        #[test]
        fn test_division_by_zero() {
            let result = calculate("10 / 0");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Деление на ноль");
        }

        #[test]
        fn test_zero_division() {
            let result = calculate("0 / 5");
            assert_eq!(result.unwrap(), 0.0);
        }

        #[test]
        fn test_large_numbers() {
            let result = calculate("1000000 * 1000000");
            assert_eq!(result.unwrap(), 1_000_000_000_000.0);
        }

        #[test]
        fn test_small_numbers() {
            let result = calculate("0.0001 * 0.0002");
            assert_eq!(result.unwrap(), 0.00000002);
        }

        // === Неверный формат ===

        #[test]
        fn test_invalid_format_missing_operator() {
            let result = calculate("5 3");
            assert!(result.is_err());
        }

        #[test]
        fn test_invalid_format_too_many_parts() {
            let result = calculate("2 + 3 + 4");
            assert!(result.is_err());
        }

        #[test]
        fn test_invalid_first_number() {
            let result = calculate("abc + 3");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("abc"));
        }

        #[test]
        fn test_invalid_second_number() {
            let result = calculate("5 + abc");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("abc"));
        }

        #[test]
        fn test_invalid_operator() {
            let result = calculate("5 % 3");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("%"));
        }

        #[test]
        fn test_empty_expression() {
            let result = calculate("");
            assert!(result.is_err());
        }

        #[test]
        fn test_only_spaces() {
            let result = calculate("   ");
            assert!(result.is_err());
        }

        // === Множественные пробелы ===

        #[test]
        fn test_multiple_spaces() {
            assert_eq!(calculate("2   +   3").unwrap(), 5.0);
            assert_eq!(calculate("  10  /  2  ").unwrap(), 5.0);
        }

        // === Специальные случаи ===

        #[test]
        fn test_power_zero() {
            assert_eq!(calculate("5 ^ 0").unwrap(), 1.0);
        }

        #[test]
        fn test_power_one() {
            assert_eq!(calculate("5 ^ 1").unwrap(), 5.0);
        }

        #[test]
        fn test_zero_power() {
            assert_eq!(calculate("0 ^ 5").unwrap(), 0.0);
        }

        #[test]
        fn test_division_result_float() {
            let result = calculate("7 / 2").unwrap();
            assert_eq!(result, 3.5);
        }
    }

    // ============ ТЕСТЫ ДЛЯ HISTORY ============

    mod history_tests {
        use super::*;

        #[test]
        fn test_history_creation() {
            let mut history: Vec<String> = Vec::new();
            assert!(history.is_empty());
            
            history.push("2 + 3 = 5".to_string());
            assert_eq!(history.len(), 1);
            assert_eq!(history[0], "2 + 3 = 5");
        }

        #[test]
        fn test_history_clear() {
            let mut history: Vec<String> = Vec::new();
            history.push("test".to_string());
            history.clear();
            assert!(history.is_empty());
        }

        #[test]
        fn test_history_multiple_entries() {
            let mut history: Vec<String> = Vec::new();
            history.push("1 + 1 = 2".to_string());
            history.push("2 + 2 = 4".to_string());
            history.push("3 + 3 = 6".to_string());
            
            assert_eq!(history.len(), 3);
            assert_eq!(history[0], "1 + 1 = 2");
            assert_eq!(history[1], "2 + 2 = 4");
            assert_eq!(history[2], "3 + 3 = 6");
        }
    }

    // ============ ТЕСТЫ ДЛЯ OPERATORS ============

    mod operator_tests {
        use super::*;

        #[test]
        fn test_all_operators() {
            let operators = ["+", "-", "*", "/", "^"];
            let results = [
                calculate("2 + 3").unwrap(),
                calculate("5 - 3").unwrap(),
                calculate("2 * 3").unwrap(),
                calculate("6 / 3").unwrap(),
                calculate("2 ^ 3").unwrap(),
            ];
            
            assert_eq!(results, [5.0, 2.0, 6.0, 2.0, 8.0]);
        }

        #[test]
        fn test_operator_precedence() {
            // В нашем калькуляторе нет приоритета операций,
            // поэтому эти тесты проверяют только базовую работу
            let result = calculate("2 + 3 * 4"); // Парсится как 2 + 3 * 4
            assert!(result.is_err()); // Неверный формат
        }
    }

    // ============ ТЕСТЫ ДЛЯ ПАРСИНГА ============

    mod parsing_tests {
        use super::*;

        #[test]
        fn test_parse_positive_integer() {
            let result = calculate("42 + 1");
            assert_eq!(result.unwrap(), 43.0);
        }

        #[test]
        fn test_parse_negative_integer() {
            let result = calculate("-10 + 5");
            assert_eq!(result.unwrap(), -5.0);
        }

        #[test]
        fn test_parse_float_with_dot() {
            let result = calculate("3.14 * 2");
            assert_eq!(result.unwrap(), 6.28);
        }

        #[test]
        fn test_parse_float_without_leading_zero() {
            let result = calculate(".5 + .5");
            assert_eq!(result.unwrap(), 1.0);
        }

        #[test]
        fn test_parse_trailing_dot() {
            let result = calculate("5. + 2");
            // Rust позволяет парсить "5." как 5.0
            assert_eq!(result.unwrap(), 7.0);
        }
    }

    // ============ ТЕСТЫ ДЛЯ ОШИБОК ============

    mod error_tests {
        use super::*;

        #[test]
        fn test_error_messages_contain_operator() {
            let result = calculate("5 % 3");
            let err = result.unwrap_err();
            assert!(err.contains("%"));
            assert!(err.contains("+"));
            assert!(err.contains("-"));
            assert!(err.contains("*"));
            assert!(err.contains("/"));
            assert!(err.contains("^"));
        }

        #[test]
        fn test_error_messages_contain_invalid_number() {
            let result = calculate("abc + 3");
            let err = result.unwrap_err();
            assert!(err.contains("abc"));
        }

        #[test]
        fn test_division_by_zero_message() {
            let result = calculate("10 / 0");
            let err = result.unwrap_err();
            assert_eq!(err, "Деление на ноль");
        }

        #[test]
        fn test_invalid_format_message() {
            let result = calculate("2 + 3 + 4");
            let err = result.unwrap_err();
            assert!(err.contains("формат"));
        }
    }

    // ============ ТЕСТЫ ДЛЯ ТОЧНОСТИ ============

    mod precision_tests {
        use super::*;

        #[test]
        fn test_precision_addition() {
            let result = calculate("0.1 + 0.2").unwrap();
            // В f64 0.1 + 0.2 = 0.30000000000000004
            assert!((result - 0.3).abs() < 0.0001);
        }

        #[test]
        fn test_precision_multiplication() {
            let result = calculate("0.1 * 0.1").unwrap();
            assert!((result - 0.01).abs() < 0.0001);
        }

        #[test]
        fn test_precision_division() {
            let result = calculate("1 / 3").unwrap();
            assert!((result - 0.3333333333).abs() < 0.0000000001);
        }
    }

    // ============ ТЕСТЫ ДЛЯ УСТОЙЧИВОСТИ ============

    mod robustness_tests {
        use super::*;

        #[test]
        fn test_very_large_number() {
            let result = calculate("1e20 + 1e20");
            assert!(result.is_ok());
        }

        #[test]
        fn test_very_small_number() {
            let result = calculate("1e-20 + 1e-20");
            assert!(result.is_ok());
        }

        #[test]
        fn test_infinity() {
            let result = calculate("1e308 * 1e308"); // Переполнение
            assert!(result.is_ok()); // В f64 это будет inf
        }

        #[test]
        fn test_division_by_zero_returns_error() {
            let result = calculate("0.0 / 0.0");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Деление на ноль");
        }
    }

}


