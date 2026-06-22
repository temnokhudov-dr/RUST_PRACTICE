#![warn(clippy::all, clippy::pedantic)]

use eframe::egui;
use std::f32::consts::PI;

pub fn run_polygon() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_min_inner_size([600.0, 500.0])
            .with_title("3D Вращающийся многогранник"),
        ..Default::default()
    };
    eframe::run_native(
        "3D Polygon",
        options,
        Box::new(|_cc| Box::new(My3DApp::default())),
    )
}

// === 3D Математика ===

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // Метод для масштабирования
    fn scale(&self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    fn rotate_x(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Vec3 {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    fn rotate_y(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Vec3 {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    fn rotate_z(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Vec3 {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }

    fn project(&self, fov: f32, distance: f32) -> egui::Pos2 {
        let factor = fov / (distance + self.z);
        egui::pos2(
            self.x * factor,
            self.y * factor,
        )
    }
}

// === Грани ===

struct Face {
    vertices: [usize; 3],
    color: egui::Color32,
}

// === Основное приложение ===

struct My3DApp {
    angle_x: f32,
    angle_y: f32,
    angle_z: f32,
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    fov: f32,
    distance: f32,
    is_rotating: bool,
    show_wireframe: bool,
    show_vertices: bool,
    shape_type: ShapeType,
    auto_rotate: bool,
}

#[derive(PartialEq, Clone, Copy)]
enum ShapeType {
    Icosahedron,
    Cube,
    Octahedron,
    Dodecahedron,
}

impl Default for My3DApp {
    fn default() -> Self {
        let mut app = Self {
            angle_x: 0.0,
            angle_y: 0.0,
            angle_z: 0.0,
            speed_x: 0.005,
            speed_y: 0.01,
            speed_z: 0.003,
            vertices: Vec::new(),
            faces: Vec::new(),
            fov: 300.0,
            distance: 500.0,
            is_rotating: true,
            show_wireframe: false,
            show_vertices: true,
            shape_type: ShapeType::Icosahedron,
            auto_rotate: true,
        };
        app.generate_shape();
        app
    }
}

impl My3DApp {
    fn generate_shape(&mut self) {
        match self.shape_type {
            ShapeType::Icosahedron => self.generate_icosahedron(),
            ShapeType::Cube => self.generate_cube(),
            ShapeType::Octahedron => self.generate_octahedron(),
            ShapeType::Dodecahedron => self.generate_dodecahedron(),
        }
    }

    fn generate_icosahedron(&mut self) {
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let scale = 120.0;
        
        self.vertices = vec![
            Vec3::new(-1.0,  phi, 0.0).scale(scale),
            Vec3::new( 1.0,  phi, 0.0).scale(scale),
            Vec3::new(-1.0, -phi, 0.0).scale(scale),
            Vec3::new( 1.0, -phi, 0.0).scale(scale),
            Vec3::new(0.0, -1.0,  phi).scale(scale),
            Vec3::new(0.0,  1.0,  phi).scale(scale),
            Vec3::new(0.0, -1.0, -phi).scale(scale),
            Vec3::new(0.0,  1.0, -phi).scale(scale),
            Vec3::new( phi, 0.0, -1.0).scale(scale),
            Vec3::new( phi, 0.0,  1.0).scale(scale),
            Vec3::new(-phi, 0.0, -1.0).scale(scale),
            Vec3::new(-phi, 0.0,  1.0).scale(scale),
        ];

        let face_indices = [
            [0, 1, 5], [0, 5, 11], [0, 11, 10], [0, 10, 7], [0, 7, 1],
            [1, 7, 8], [1, 8, 9], [1, 9, 5], [5, 9, 4], [5, 4, 11],
            [11, 4, 2], [11, 2, 10], [10, 2, 6], [10, 6, 7], [7, 6, 8],
            [8, 6, 3], [8, 3, 9], [9, 3, 4], [4, 3, 2], [2, 3, 6],
        ];

        self.faces = face_indices.iter().enumerate().map(|(i, &indices)| {
            let hue = (i as f32 / face_indices.len() as f32) * 360.0;
            Face {
                vertices: [indices[0], indices[1], indices[2]],
                color: Self::hsl_to_color(hue, 0.8, 0.6),
            }
        }).collect();
    }

    fn generate_cube(&mut self) {
        let s = 100.0;
        self.vertices = vec![
            Vec3::new(-s, -s, -s), Vec3::new( s, -s, -s),
            Vec3::new( s,  s, -s), Vec3::new(-s,  s, -s),
            Vec3::new(-s, -s,  s), Vec3::new( s, -s,  s),
            Vec3::new( s,  s,  s), Vec3::new(-s,  s,  s),
        ];

        let faces = [
            [0,1,2,3], [4,5,6,7], [0,1,5,4],
            [2,3,7,6], [0,3,7,4], [1,2,6,5],
        ];

        self.faces = faces.iter().enumerate().map(|(i, &indices)| {
            Face {
                vertices: [indices[0], indices[1], indices[2]],
                color: match i {
                    0 => egui::Color32::from_rgb(255, 100, 100),
                    1 => egui::Color32::from_rgb(100, 255, 100),
                    2 => egui::Color32::from_rgb(100, 100, 255),
                    3 => egui::Color32::from_rgb(255, 255, 100),
                    4 => egui::Color32::from_rgb(255, 100, 255),
                    5 => egui::Color32::from_rgb(100, 255, 255),
                    _ => egui::Color32::GRAY,
                },
            }
        }).collect();
    }

    fn generate_octahedron(&mut self) {
        let s = 120.0;
        self.vertices = vec![
            Vec3::new( s, 0.0, 0.0), Vec3::new(-s, 0.0, 0.0),
            Vec3::new(0.0,  s, 0.0), Vec3::new(0.0, -s, 0.0),
            Vec3::new(0.0, 0.0,  s), Vec3::new(0.0, 0.0, -s),
        ];

        let faces = [
            [0,2,4], [0,4,3], [0,3,5], [0,5,2],
            [1,4,2], [1,3,4], [1,5,3], [1,2,5],
        ];

        self.faces = faces.iter().enumerate().map(|(i, &indices)| {
            let hue = (i as f32 / faces.len() as f32) * 360.0;
            Face {
                vertices: [indices[0], indices[1], indices[2]],
                color: Self::hsl_to_color(hue, 0.9, 0.5),
            }
        }).collect();
    }

    fn generate_dodecahedron(&mut self) {
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let scale = 70.0;
        
        let vertices_raw = [
            ( 1.0,  1.0,  1.0), ( 1.0,  1.0, -1.0), ( 1.0, -1.0,  1.0),
            ( 1.0, -1.0, -1.0), (-1.0,  1.0,  1.0), (-1.0,  1.0, -1.0),
            (-1.0, -1.0,  1.0), (-1.0, -1.0, -1.0),
            ( 0.0,  1.0/phi,  phi), ( 0.0, -1.0/phi,  phi),
            ( 0.0,  1.0/phi, -phi), ( 0.0, -1.0/phi, -phi),
            ( 1.0/phi,  phi, 0.0), (-1.0/phi,  phi, 0.0),
            ( 1.0/phi, -phi, 0.0), (-1.0/phi, -phi, 0.0),
            ( phi, 0.0,  1.0/phi), ( phi, 0.0, -1.0/phi),
            (-phi, 0.0,  1.0/phi), (-phi, 0.0, -1.0/phi),
        ];

        self.vertices = vertices_raw.iter()
            .map(|(x, y, z)| Vec3::new(*x, *y, *z).scale(scale))
            .collect();

        let faces = [
            [0,12,13,4,8], [0,8,9,2,16], [0,16,17,1,12],
            [1,17,18,5,13], [2,9,10,3,14], [3,10,11,7,15],
            [4,13,5,18,19], [5,19,6,11,10], [6,19,18,17,16],
            [6,16,2,14,15], [7,11,3,14,15], [7,19,18,17,16],
        ];

        self.faces = faces.iter().take(10).enumerate().map(|(i, &indices)| {
            Face {
                vertices: [indices[0], indices[1], indices[2]],
                color: match i % 6 {
                    0 => egui::Color32::from_rgb(255, 200, 150),
                    1 => egui::Color32::from_rgb(150, 255, 200),
                    2 => egui::Color32::from_rgb(200, 150, 255),
                    3 => egui::Color32::from_rgb(255, 150, 200),
                    4 => egui::Color32::from_rgb(200, 255, 150),
                    _ => egui::Color32::from_rgb(150, 200, 255),
                },
            }
        }).collect();
    }

    // Исправленная функция с более понятными именами
    #[allow(clippy::many_single_char_names)]
    fn hsl_to_color(hue: f32, saturation: f32, lightness: f32) -> egui::Color32 {
        let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = lightness - chroma / 2.0;
        
        let (red, green, blue) = if hue < 60.0 {
            (chroma, x, 0.0)
        } else if hue < 120.0 {
            (x, chroma, 0.0)
        } else if hue < 180.0 {
            (0.0, chroma, x)
        } else if hue < 240.0 {
            (0.0, x, chroma)
        } else if hue < 300.0 {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };
        
        egui::Color32::from_rgb(
            ((red + m) * 255.0) as u8,
            ((green + m) * 255.0) as u8,
            ((blue + m) * 255.0) as u8,
        )
    }

    fn draw_3d_scene(&self, painter: &egui::Painter, rect: egui::Rect) {
        let center = rect.center();
        
        let rotated_vertices: Vec<Vec3> = self.vertices.iter()
            .map(|v| {
                let mut v = *v;
                if self.auto_rotate {
                    v = v.rotate_x(self.angle_x);
                    v = v.rotate_y(self.angle_y);
                    v = v.rotate_z(self.angle_z);
                }
                v
            })
            .collect();

        let projected: Vec<egui::Pos2> = rotated_vertices.iter()
            .map(|v| {
                let p = v.project(self.fov, self.distance);
                egui::pos2(center.x + p.x, center.y + p.y)
            })
            .collect();

        let mut sorted_faces: Vec<(usize, f32)> = self.faces.iter()
            .enumerate()
            .map(|(i, face)| {
                let v0 = rotated_vertices[face.vertices[0]];
                let v1 = rotated_vertices[face.vertices[1]];
                let v2 = rotated_vertices[face.vertices[2]];
                let avg_z = (v0.z + v1.z + v2.z) / 3.0;
                (i, avg_z)
            })
            .collect();
        sorted_faces.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (face_idx, _) in sorted_faces {
            let face = &self.faces[face_idx];
            let pts: Vec<egui::Pos2> = face.vertices.iter()
                .map(|&idx| projected[idx])
                .collect();

            if pts.len() >= 3 {
                if !self.show_wireframe {
                    let fill_shape = egui::Shape::convex_polygon(
                        pts.clone(),
                        face.color,
                        egui::Stroke::NONE,
                    );
                    painter.add(fill_shape);
                }

                painter.line_segment([pts[0], pts[1]], egui::Stroke::new(1.5, egui::Color32::WHITE));
                painter.line_segment([pts[1], pts[2]], egui::Stroke::new(1.5, egui::Color32::WHITE));
                painter.line_segment([pts[2], pts[0]], egui::Stroke::new(1.5, egui::Color32::WHITE));
            }
        }

        if self.show_vertices {
            for (i, &pos) in projected.iter().enumerate() {
                painter.circle_filled(pos, 4.0, egui::Color32::YELLOW);
                painter.text(
                    pos + egui::vec2(6.0, -6.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("v{}", i),
                    egui::FontId::proportional(10.0),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

impl eframe::App for My3DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_rotating {
            self.angle_x += self.speed_x;
            self.angle_y += self.speed_y;
            self.angle_z += self.speed_z;
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎲 3D Вращающийся многогранник");
                ui.add_space(20.0);
                ui.label(format!("Вершин: {}", self.vertices.len()));
                ui.label(format!("Граней: {}", self.faces.len()));
            });
            ui.separator();

            let (rect_response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::drag()
            );
            let rect = rect_response.rect;

            painter.rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(10, 10, 30),
            );

            for i in 0..30 {
                let pos = i as f32 / 30.0;
                let x = rect.left() + rect.width() * pos;
                let y = rect.top() + rect.height() * pos;
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    egui::Stroke::new(0.3, egui::Color32::from_rgba_premultiplied(50, 50, 100, 30)),
                );
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    egui::Stroke::new(0.3, egui::Color32::from_rgba_premultiplied(50, 50, 100, 30)),
                );
            }

            self.draw_3d_scene(&painter, rect);

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button(if self.is_rotating { "⏸ Пауза" } else { "▶ Запустить" }).clicked() {
                    self.is_rotating = !self.is_rotating;
                }
                
                ui.add_space(10.0);
                
                if ui.button("🔄 Сброс угла").clicked() {
                    self.angle_x = 0.0;
                    self.angle_y = 0.0;
                    self.angle_z = 0.0;
                }

                ui.add_space(10.0);

                ui.checkbox(&mut self.show_vertices, "Вершины");
                ui.checkbox(&mut self.show_wireframe, "Каркас");
            });

            ui.horizontal(|ui| {
                ui.label("Скорость X:");
                ui.add(egui::Slider::new(&mut self.speed_x, 0.0..=0.02).text(""));
                ui.label("Y:");
                ui.add(egui::Slider::new(&mut self.speed_y, 0.0..=0.02).text(""));
                ui.label("Z:");
                ui.add(egui::Slider::new(&mut self.speed_z, 0.0..=0.02).text(""));
            });

            ui.horizontal(|ui| {
                ui.label("Фигура:");
                // Исправленный ComboBox
                let current_text = match self.shape_type {
                    ShapeType::Icosahedron => "Икосаэдр",
                    ShapeType::Cube => "Куб",
                    ShapeType::Octahedron => "Октаэдр",
                    ShapeType::Dodecahedron => "Додекаэдр",
                };
                egui::ComboBox::from_label("")
                    .selected_text(current_text)
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(self.shape_type == ShapeType::Icosahedron, "Икосаэдр").clicked() {
                            self.shape_type = ShapeType::Icosahedron;
                            self.generate_shape();
                        }
                        if ui.selectable_label(self.shape_type == ShapeType::Cube, "Куб").clicked() {
                            self.shape_type = ShapeType::Cube;
                            self.generate_shape();
                        }
                        if ui.selectable_label(self.shape_type == ShapeType::Octahedron, "Октаэдр").clicked() {
                            self.shape_type = ShapeType::Octahedron;
                            self.generate_shape();
                        }
                        if ui.selectable_label(self.shape_type == ShapeType::Dodecahedron, "Додекаэдр").clicked() {
                            self.shape_type = ShapeType::Dodecahedron;
                            self.generate_shape();
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("FOV:");
                ui.add(egui::Slider::new(&mut self.fov, 100.0..=600.0).text(""));
                ui.label("Дистанция:");
                ui.add(egui::Slider::new(&mut self.distance, 200.0..=1000.0).text(""));
            });

            if ui.button("❌ Выход").clicked() {
                std::process::exit(0);
            }
        });
    }
}