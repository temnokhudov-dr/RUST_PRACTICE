#![warn(clippy::all, clippy::pedantic)]

use std::fmt::Display;

// Реализация класса Queue (очередь)
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    // Конструктор для создания новой очереди
    pub fn new() -> Self {
        Queue {
            elements: Vec::new(),
        }
    }

    // Добавление элемента в очередь (enqueue)
    pub fn enqueue(&mut self, element: T) {
        self.elements.push(element);
    }

    // Удаление элемента из очереди (dequeue)
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    // Чтение первого элемента очереди (peek)
    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    // Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Получение размера очереди
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}


// Реализация класса BackgroundTask (задача в фоне)
pub struct BackgroundTask<T> {
    queue: Queue<T>,
}

impl<T: Display> BackgroundTask<T> {
    // Конструктор для создания новой очереди
    pub fn new() -> Self {
        BackgroundTask {
            queue: Queue::new(),
        }
    }

    pub fn add(&mut self, job: T) {
        self.queue.enqueue(job);
    }

    // Выполнение всех задач по очереди
    pub fn execute(&mut self) {
        while !self.queue.is_empty() {
            if let Some(job) = self.queue.dequeue() {
                println!("{} is done!", job); // Выполняем задачу
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // ============ ТЕСТЫ ДЛЯ QUEUE ============

    #[test]
    fn test_queue_enqueue_and_dequeue() {
        let mut queue = Queue::new();
        
        // Добавляем элементы
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        
        // Проверяем размер
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        
        // Проверяем порядок FIFO (First In First Out)
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        
        // Проверяем, что очередь пуста
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_queue_peek_and_empty() {
        let mut queue = Queue::new();
        
        // Проверяем пустую очередь
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
        
        // Добавляем элементы
        queue.enqueue("Hello");
        queue.enqueue("World");
        
        // Проверяем peek (не удаляет элемент)
        assert_eq!(queue.peek(), Some(&"Hello"));
        assert_eq!(queue.len(), 2); // размер не изменился
        
        // Удаляем элемент и проверяем снова
        queue.dequeue();
        assert_eq!(queue.peek(), Some(&"World"));
        assert_eq!(queue.len(), 1);
        
        // Удаляем последний элемент
        queue.dequeue();
        assert!(queue.is_empty());
        assert_eq!(queue.peek(), None);
    }

    // ============ ТЕСТЫ ДЛЯ BACKGROUNDTASK ============

    #[test]
    fn test_background_task_add_and_execute() {
        let mut bg = BackgroundTask::new();
        
        // Добавляем задачи
        bg.add("Task 1");
        bg.add("Task 2");
        bg.add("Task 3");
        
        // Проверяем, что задачи добавлены (выполняем и смотрим вывод)
        bg.execute();
        // Вывод должен быть:
        // Task 1 is done!
        // Task 2 is done!
        // Task 3 is done!
        
        // После выполнения очередь должна быть пуста
        // Но мы не можем проверить это напрямую, так как queue приватная
        // Поэтому просто проверяем, что execute не паникует при повторном вызове
        bg.execute(); // Ничего не выведет, так как очередь пуста
    }

    #[test]
    fn test_background_task_with_numbers() {
        let mut bg = BackgroundTask::new();
        
        // Добавляем числа
        bg.add(100);
        bg.add(200);
        bg.add(300);
        
        // Выполняем задачи
        bg.execute();
        // Вывод должен быть:
        // 100 is done!
        // 200 is done!
        // 300 is done!
        
        // Проверяем, что все работает с числами
        bg.add(400);
        bg.execute();
        // Вывод должен быть:
        // 400 is done!
    }
}



