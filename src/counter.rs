#![warn(clippy::all, clippy::pedantic)]

use eframe::egui;


pub fn run_counter() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Мой счётчик",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}


struct MyApp {
    counter: i32,
    text_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: 0,
            text_input: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Запрашиваем перерисовку каждый кадр (для анимации)
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦀 Простое приложение на Rust");
            ui.separator();

            // === Секция счётчика ===
            ui.horizontal(|ui| {
                ui.label("Счётчик:");
                ui.label(self.counter.to_string());
            });

            ui.horizontal(|ui| {
                if ui.button("➕ Увеличить").clicked() {
                    self.counter += 1;
                }
                if ui.button("➖ Уменьшить").clicked() {
                    self.counter -= 1;
                }
                if ui.button("🔄 Сброс").clicked() {
                    self.counter = 0;
                }
            });

            // Цветовая индикация на основе значения счётчика
            let color = if self.counter > 5 {
                egui::Color32::GREEN
            } else if self.counter < -5 {
                egui::Color32::RED
            } else {
                egui::Color32::GRAY
            };

            ui.colored_label(
                color,
                format!("Состояние: {}", 
                    if self.counter > 5 { "🔥 Огонь!" }
                    else if self.counter < -5 { "❄️ Холодно!" }
                    else { "✅ Нормально" }
                )
            );

            ui.separator();

            // === Секция ввода текста ===
            ui.label("Введите текст:");
            let response = ui.text_edit_singleline(&mut self.text_input);
            if response.changed() {
                // Действие при изменении текста
            }

            if !self.text_input.is_empty() {
                ui.label(format!("Вы ввели: {}", self.text_input));
            }

            ui.separator();

            // === Кнопка выхода ===
            if ui.button("❌ Выход").clicked() {
                std::process::exit(0);
            }
        });
    }
}






