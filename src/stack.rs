use std::collections::HashMap; 

// Реализация класса Stack
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // Конструктор для создания нового стека
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // Создание с элементами 
    pub fn with_elements(elements: Vec<T>) -> Self {
        Stack { elements }
    }

    // Чтение верхнего элемента (peek)
    pub fn read(&self) -> Option<&T> {
        self.elements.last()
    }

    // Добавление элементов (push)
    pub fn push(&mut self, els: Vec<T>) {
        self.elements.extend(els);
    }

    // Добавление одного элемента
    pub fn push_one(&mut self, el: T) {
        self.elements.push(el);
    }

    // Удаление верхнего элемента (pop)
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Получение размера стека
    pub fn len(&self) -> usize{
        self.elements.len()
    }
}


// Реализация класса Bracketer

pub struct Bracketer {
    bracket_pairs: std::collections::HashMap<char, char>,
}

impl Bracketer {
    pub fn new() -> Self {
        let mut pairs = std::collections::HashMap::new();
        pairs.insert(')', '(');
        pairs.insert('}', '{');
        pairs.insert(']', '[');
        
        Bracketer {
            bracket_pairs: pairs,
        }
    }
    
    pub fn check(&self, text: &str) -> bool {
        let mut stack = Stack::new();
        
        for (i, current_char) in text.char_indices() {
            // Проверяем, является ли символ открывающей скобкой
            if self.is_opening_bracket(current_char) {
                stack.push_one(current_char);
            }
            // Проверяем, является ли символ закрывающей скобкой
            else if self.is_closing_bracket(current_char) {
                match stack.pop() {
                    Some(last_open) => {
                        // Проверяем, соответствует ли последняя открытая скобка
                        // этой закрывающей
                        if !self.is_matching_pair(last_open, current_char) {
                            println!("Ошибка на позиции {}: '{}' не соответствует '{}'", 
                                     i, current_char, last_open);
                            return false;
                        }
                    }
                    None => {
                        println!("Ошибка на позиции {}: закрывающая скобка '{}' без открывающей", 
                                 i, current_char);
                        return false;
                    }
                }
            }
            // Все остальные символы игнорируем
        }
        
        // После проверки всех символов стек должен быть пустым
        if !stack.is_empty() {
            println!("Ошибка: остались незакрытые скобки ({} шт.)", stack.len());
            return false;
        }
        
        true
    }
    
    // Проверяет, является ли символ открывающей скобкой
    fn is_opening_bracket(&self, ch: char) -> bool {
        ch == '(' || ch == '{' || ch == '['
    }
    
    // Проверяет, является ли символ закрывающей скобкой
    fn is_closing_bracket(&self, ch: char) -> bool {
        ch == ')' || ch == '}' || ch == ']'
    }
    
    // Проверяет, соответствуют ли открывающая и закрывающая скобки друг другу
    fn is_matching_pair(&self, open: char, close: char) -> bool {
        match (open, close) {
            ('(', ')') => true,
            ('{', '}') => true,
            ('[', ']') => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_sequences() {
        let bracketer = Bracketer::new();
        
        assert!(bracketer.check("()"));
        assert!(bracketer.check("()[]{}"));
        assert!(bracketer.check("([])"));
        assert!(bracketer.check("{[()]}"));
        assert!(bracketer.check("(hello {world} [test])"));
        assert!(bracketer.check("Demo. (internal {demo2} [valid])"));
    }
    
    #[test]
    fn test_invalid_sequences() {
        let bracketer = Bracketer::new();
        
        assert!(!bracketer.check("(]"));
        assert!(!bracketer.check("([)]"));
        assert!(!bracketer.check("{"));
        assert!(!bracketer.check(")"));
        assert!(!bracketer.check("(("));
        assert!(!bracketer.check("Demo. (internal {demo2} [invalid)"));
    }
}


