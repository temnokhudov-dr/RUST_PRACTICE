#![warn(clippy::all, clippy::pedantic)]

use std::env;  // Для работы с окружением и аргументами командной строки
use std::fs;   // Для работы с файловой системой (создание директорий)
use rdev::{listen, grab, Event, EventType, Key};
use screenshots::Screen;
use chrono::Local;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::io::{self, Write};
use std::thread;

// Константа с именем директории по умолчанию
const TARGET_DIR: &str = "screens";

pub fn run_screen_app() -> std::io::Result<()> {

    println!("==============================");
    println!(" Screen Capture Tool");
    println!("==============================");

    request_screen_capture_permission();

    // Собираем все аргументы командной строки в вектор строк
    // args[0] - имя программы, args[1] - первый аргумент пользователя и т.д.
    let args: Vec<String> = env::args().collect();

    // Определяем имя директории для создания
    // Если пользователь передал аргумент (args[1]) - используем его
    // Иначе используем значение по умолчанию из константы TARGET_DIR
    let screens_dir = args
        .get(1)
        .unwrap_or(&TARGET_DIR.to_string())
        .to_string();

    // Получаем текущую рабочую директорию и создаем директорию на диске
    let root_path = choose_root_path(&screens_dir)?;

    await_keyboard_release(1000);

    fs::create_dir_all(&root_path)?;
    println!("Директория создана: {:?}", root_path);

    let running = Arc::new(AtomicBool::new(true));
    let grab_running = running.clone();
    let root_clone = root_path.clone();

    println!("==============================");
    println!("Controls:");
    println!("  P        -> make screenshot");
    println!("  Escape   -> exit program");
    println!();
    println!("==============================");

    thread::spawn(move || {
        let _ = grab(move |e| {
            callback(e, &root_clone, &grab_running)
        });
    });

    while running.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    exit_prompt(&root_path);

    Ok(())
}

fn callback(
    event: Event,
    root_path: &std::path::PathBuf,
    running: &Arc<AtomicBool>,
) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::KeyP) => {
            let _ = make_screen(root_path);
            None
        }

        EventType::KeyPress(Key::Escape) => {
            println!("🛑 Exit...");
            running.store(false, Ordering::Relaxed);
            None
        }

        _ => Some(event),
    }
}

fn make_screen(
root_path: &std::path::PathBuf
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    make_screen_internal(root_path)
}

fn make_screen_internal(
    root_path: &std::path::PathBuf
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let screens = Screen::all()?;

    if screens.is_empty() {
        return Err("Мониторы не найдены".into());
    }

    let timestamp = Local::now()
        .format("%Y-%m-%d_%H-%M-%S")
        .to_string();

    let session_dir = root_path.join(&timestamp);
    fs::create_dir_all(&session_dir)?;

    for (index, screen) in screens.iter().enumerate() {
        let image = screen.capture()?;

        let filename = session_dir.join(
            format!("monitor_{}.png", index + 1)
        );

        image.save(&filename)?;
        println!("✅ Скриншот сохранён: {:?}", filename);
    }

    Ok(session_dir)
}

fn request_screen_capture_permission() {
    println!("Запрашиваем разрешение на захват экрана...");

    match Screen::all() {
        Ok(screens) if !screens.is_empty() => {
            match screens[0].capture() {
                Ok(_) => println!("✅ Разрешение на захват экрана получено."),
                Err(e) => eprintln!("❌ Не удалось получить разрешение: {e}"),
            }
        }
        Ok(_) => {
            eprintln!("❌ Не найдено ни одного монитора.");
        }
        Err(e) => {
            eprintln!("❌ Ошибка получения списка мониторов: {e}");
        }
    }
}

fn exit_prompt(root_path: &std::path::PathBuf) {
    println!("\n==============================");
    println!("Сохранить папку со скриншотами?");
    println!("Путь: {:?}", root_path);
    println!("[Y] сохранить / [N] удалить");

    print!("> ");
    io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" => {
                println!("📁 Сохранено");
                break;
            }

            "n" => {
                println!("🗑 Удаляем папку {:?}...", root_path);
                let _ = fs::remove_dir_all(root_path);
                break;
            }

            _ => println!("Введите Y / N"),
        }
    }
}

fn choose_root_path(default_dir: &str) -> std::io::Result<std::path::PathBuf> {
    let cwd = std::env::current_dir()?;

    println!("\n==============================");
    println!("Выбор директории сохранения");
    println!("==============================");
    println!("Текущая директория: {:?}", cwd);
    println!("[Y] использовать её");
    println!("[N] ввести свою");
    println!("[ESC] выход");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" => {
                let mut path = cwd.clone();
                path.push(choose_screens_dir_name(default_dir));
                return Ok(path);
            }

            "n" => {
                loop {
                    println!("Введите путь: ");

                    let mut path_input = String::new();
                    io::stdin().read_line(&mut path_input).unwrap();

                    let mut path = std::path::PathBuf::from(path_input.trim());

                    if path.is_dir() {
                        path.push(choose_screens_dir_name(default_dir));
                        return Ok(path);
                    } else {
                        println!("❌ Путь не существует. Попробуйте ещё раз или Ctrl+C");
                    }
                }
            }

            "esc" => {
                println!("Выход...");
                std::process::exit(0);
            }

            _ => {
                println!("Введите Y / N / ESC");
            }
        }
    }
}

fn choose_screens_dir_name(default_dir: &str) -> String {
    println!("\n==============================");
    println!("Выбор имени папки для сохранения скриншотов");
    println!("==============================");
    println!("Текущее имя папки: {}", default_dir);
    println!("[Y] использовать текущее имя");
    println!("[N] ввести новое имя");
    println!("[ESC] выход");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" => {
                return default_dir.to_string();
            }

            "n" => {
                loop {
                    print!("Введите новое имя папки: ");
                    io::stdout().flush().unwrap();

                    let mut input_dir = String::new();
                    io::stdin().read_line(&mut input_dir).unwrap();

                    let input_dir = input_dir.trim();

                    // Проверка на пустое имя
                    if input_dir.is_empty() {
                        println!("❌ Имя папки не может быть пустым.");
                        continue;
                    }

                    // Проверка недопустимых символов
                    if input_dir.contains(['<', '>', ':', '"', '/', '\\', '|', '?', '*']) {
                        println!("❌ Имя папки содержит недопустимые символы.");
                        println!("Запрещены символы: < > : \" / \\ | ? *");
                        continue;
                    }

                    return input_dir.to_string();
                }
            }

            "esc" => {
                println!("Выход...");
                std::process::exit(0);
            }

            _ => {
                println!("Введите Y / N / ESC");
            }
        }
    }
}

fn await_keyboard_release(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}








#[cfg(test)]
mod tests {
    use super::*;
    use rdev::{Event, EventType, Key};
    use std::path::PathBuf;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    #[test]
    fn callback_escape_sets_running_false() {
        let running = Arc::new(AtomicBool::new(true));
        let root = PathBuf::from("screens");

        let event = Event {
            time: std::time::SystemTime::now(),
            name: None,
            event_type: EventType::KeyPress(Key::Escape),
        };

        let result = callback(event, &root, &running);

        assert!(result.is_none());
        assert!(!running.load(Ordering::Relaxed));
    }

    #[test]
    fn callback_other_key_is_not_blocked() {
        let running = Arc::new(AtomicBool::new(true));
        let root = PathBuf::from("screens");

        let event = Event {
            time: std::time::SystemTime::now(),
            name: None,
            event_type: EventType::KeyPress(Key::KeyA),
        };

        let result = callback(event, &root, &running);

        assert!(result.is_some());
        assert!(running.load(Ordering::Relaxed));
    }

    #[test]
    fn await_keyboard_release_waits() {
        use std::time::{Duration, Instant};

        let start = Instant::now();

        await_keyboard_release(150);

        assert!(start.elapsed() >= Duration::from_millis(150));
    }

        #[test]
    fn default_directory_name() {
        assert_eq!(TARGET_DIR, "screens");
    }

}


