#![warn(clippy::all, clippy::pedantic)]

use eframe::egui;
use std::f32::consts::PI;

pub fn run_polygon() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([400.0, 400.0])
            .with_title("Вращающийся многоугольник"),
        ..Default::default()
    };
    eframe::run_native(
        "Polygon Rotator",
        options,
        Box::new(|_cc| Box::new(PolygonApp::default())),
    )
}

struct PolygonApp {
    angle: f32,
    speed: f32,
    sides: usize,
    radius: f32,
    color: egui::Color32,
    stroke_color: egui::Color32,
    show_vertices: bool,
    is_rotating: bool,
}

impl Default for PolygonApp {
    fn default() -> Self {
        Self {
            angle: 0.0,
            speed: 0.02,
            sides: 6,
            radius: 100.0,
            color: egui::Color32::from_rgb(100, 200, 255),
            stroke_color: egui::Color32::from_rgb(50, 50, 150),
            show_vertices: true,
            is_rotating: true,
        }
    }
}

impl PolygonApp {
    fn get_vertices(&self, center: egui::Pos2) -> Vec<egui::Pos2> {
        let mut vertices = Vec::with_capacity(self.sides);
        
        for i in 0..self.sides {
            let angle = self.angle + (i as f32 / self.sides as f32) * 2.0 * PI;
            let x = center.x + self.radius * angle.cos();
            let y = center.y + self.radius * angle.sin();
            vertices.push(egui::pos2(x, y));
        }
        
        vertices
    }

    fn draw_polygon(&self, painter: &egui::Painter, center: egui::Pos2) {
        let vertices = self.get_vertices(center);
        
        if vertices.len() < 3 {
            return;
        }

        let fill_shape = egui::Shape::convex_polygon(
            vertices.clone(),
            self.color,
            egui::Stroke::new(2.0, self.stroke_color),
        );
        painter.add(fill_shape);

        if self.show_vertices {
            for &vertex in &vertices {
                painter.circle_filled(
                    vertex,
                    5.0,
                    egui::Color32::from_rgb(255, 100, 100),
                );
                let label = format!("({:.0}, {:.0})", vertex.x, vertex.y);
                painter.text(
                    vertex + egui::vec2(8.0, -8.0),
                    egui::Align2::LEFT_BOTTOM,
                    label,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
            }
        }

        painter.circle_filled(
            center,
            6.0,
            egui::Color32::from_rgb(255, 255, 100),
        );
        
        painter.text(
            center + egui::vec2(10.0, 10.0),
            egui::Align2::LEFT_TOP,
            "Центр",
            egui::FontId::default(),
            egui::Color32::YELLOW,
        );
    }

    fn draw_angle_indicator(&self, painter: &egui::Painter, center: egui::Pos2) {
        painter.circle_stroke(
            center,
            self.radius + 30.0,
            egui::Stroke::new(1.0, egui::Color32::GRAY),
        );

        for i in 0..12 {
            let angle = (i as f32 / 12.0) * 2.0 * PI;
            let inner_radius = self.radius + 25.0;
            let outer_radius = self.radius + 30.0;
            
            let inner = egui::pos2(
                center.x + inner_radius * angle.cos(),
                center.y + inner_radius * angle.sin(),
            );
            let outer = egui::pos2(
                center.x + outer_radius * angle.cos(),
                center.y + outer_radius * angle.sin(),
            );
            
            painter.line_segment([inner, outer], egui::Stroke::new(1.0, egui::Color32::GRAY));
        }
    }
}

impl eframe::App for PolygonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_rotating {
            self.angle += self.speed;
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🔷 Вращающийся многоугольник");
                ui.add_space(20.0);
                ui.label(format!("Угол: {:.2}°", self.angle.to_degrees() % 360.0));
                ui.label(format!("Сторон: {}", self.sides));
            });
            
            ui.separator();

            // ВАЖНО: используем .rect для доступа к геометрии
            let (rect_response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::drag()
            );
            
            let rect = rect_response.rect; // Получаем Rect из Response
            let center = rect.center();
            let radius = self.radius.min(rect.width().min(rect.height()) / 2.5);
            self.radius = radius;

            painter.rect_filled(
                rect, // Теперь это Rect, а не Response
                0.0,
                egui::Color32::from_rgb(20, 20, 40),
            );

            // Сетка
            for i in 0..20 {
                let pos = i as f32 / 20.0;
                let x = rect.left() + rect.width() * pos;
                let y = rect.top() + rect.height() * pos;
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(100, 100, 100, 50)),
                );
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(100, 100, 100, 50)),
                );
            }

            self.draw_angle_indicator(&painter, center);
            self.draw_polygon(&painter, center);

            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button(if self.is_rotating { "⏸ Пауза" } else { "▶ Запустить" }).clicked() {
                    self.is_rotating = !self.is_rotating;
                }
                
                ui.add_space(10.0);
                
                if ui.button("🔄 Сброс угла").clicked() {
                    self.angle = 0.0;
                }

                ui.add_space(10.0);

                ui.checkbox(&mut self.show_vertices, "Показывать вершины");
            });

            ui.horizontal(|ui| {
                ui.label("Скорость:");
                ui.add(egui::Slider::new(&mut self.speed, 0.0..=0.1).text("рад/кадр"));
            });

            ui.horizontal(|ui| {
                ui.label("Стороны:");
                ui.add(egui::Slider::new(&mut self.sides, 3..=12).text("шт"));
            });

            if ui.button("❌ Выход").clicked() {
                std::process::exit(0);
            }
        });
    }
}