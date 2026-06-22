#![warn(clippy::all, clippy::pedantic)]

use eframe::egui;
use std::f32::consts::PI;

pub fn run_3d_parametric() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 750.0])
            .with_min_inner_size([700.0, 600.0])
            .with_title("3D Параметрический конструктор"),
        ..Default::default()
    };
    eframe::run_native(
        "3D Parametric",
        options,
        Box::new(|_cc| Box::new(ParametricApp::default())),
    )
}

// === 3D Математика (та же, что и раньше) ===

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
        egui::pos2(self.x * factor, self.y * factor)
    }
}

// === Структуры данных ===

#[derive(Clone)]
struct Face {
    vertices: Vec<usize>,
    color: egui::Color32,
}

#[derive(Clone, Copy, PartialEq)]
enum PrimitiveType {
    Sphere,
    Cylinder,
    Cone,
    Torus,
    Pyramid,
    Prism,
    Geodesic,
}

impl PrimitiveType {
    fn name(&self) -> &'static str {
        match self {
            Self::Sphere => "Сфера",
            Self::Cylinder => "Цилиндр",
            Self::Cone => "Конус",
            Self::Torus => "Тор",
            Self::Pyramid => "Пирамида",
            Self::Prism => "Призма",
            Self::Geodesic => "Геодезическая",
        }
    }
}

// === Основное приложение ===

struct ParametricApp {
    // Вращение
    angle_x: f32,
    angle_y: f32,
    angle_z: f32,
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
    is_rotating: bool,
    auto_rotate: bool,

    // Параметры фигуры
    primitive_type: PrimitiveType,
    segments: usize,        // Количество сегментов по окружности
    height_segments: usize, // Количество сегментов по высоте
    radius: f32,
    height: f32,
    twist: f32,            // Скручивание для спиральных эффектов
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,

    // Визуализация
    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    fov: f32,
    distance: f32,
    show_wireframe: bool,
    show_vertices: bool,
    show_normals: bool,
    color_mode: ColorMode,
    smooth_shading: bool,
}

#[derive(PartialEq)]
enum ColorMode {
    Solid,
    Rainbow,
    Height,
    Random,
}

impl Default for ParametricApp {
    fn default() -> Self {
        let mut app = Self {
            angle_x: 0.0,
            angle_y: 0.0,
            angle_z: 0.0,
            speed_x: 0.005,
            speed_y: 0.008,
            speed_z: 0.003,
            is_rotating: true,
            auto_rotate: true,

            primitive_type: PrimitiveType::Sphere,
            segments: 12,
            height_segments: 8,
            radius: 100.0,
            height: 150.0,
            twist: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,

            vertices: Vec::new(),
            faces: Vec::new(),
            fov: 300.0,
            distance: 500.0,
            show_wireframe: false,
            show_vertices: true,
            show_normals: false,
            color_mode: ColorMode::Rainbow,
            smooth_shading: false,
        };
        app.generate_mesh();
        app
    }
}

impl ParametricApp {
    // === Генерация меша ===

    fn generate_mesh(&mut self) {
        match self.primitive_type {
            PrimitiveType::Sphere => self.generate_sphere(),
            PrimitiveType::Cylinder => self.generate_cylinder(),
            PrimitiveType::Cone => self.generate_cone(),
            PrimitiveType::Torus => self.generate_torus(),
            PrimitiveType::Pyramid => self.generate_pyramid(),
            PrimitiveType::Prism => self.generate_prism(),
            PrimitiveType::Geodesic => self.generate_geodesic(),
        }
    }

    // --- Сфера ---
    fn generate_sphere(&mut self) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let segs = self.segments;
        let h_segs = self.height_segments;

        // Вершины
        for j in 0..=h_segs {
            let theta = (j as f32 / h_segs as f32) * PI;
            let y = self.radius * theta.cos();
            let r = self.radius * theta.sin();

            for i in 0..=segs {
                let phi = (i as f32 / segs as f32) * 2.0 * PI;
                let x = r * phi.cos();
                let z = r * phi.sin();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        // Грани (треугольники)
        for j in 0..h_segs {
            for i in 0..segs {
                let a = j * (segs + 1) + i;
                let b = a + 1;
                let c = (j + 1) * (segs + 1) + i;
                let d = c + 1;

                let color = self.get_face_color(j as f32 / h_segs as f32, i as f32 / segs as f32);
                faces.push(Face { vertices: vec![a, b, c], color });
                faces.push(Face { vertices: vec![b, d, c], color });
            }
        }

        self.vertices = vertices;
        self.faces = faces;
        self.apply_transform();
    }

    // --- Цилиндр ---
    fn generate_cylinder(&mut self) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let segs = self.segments;
        let h_segs = self.height_segments;

        // Боковые вершины
        for j in 0..=h_segs {
            let y = -self.height / 2.0 + (j as f32 / h_segs as f32) * self.height;
            let twist_angle = self.twist * (j as f32 / h_segs as f32);

            for i in 0..=segs {
                let phi = (i as f32 / segs as f32) * 2.0 * PI + twist_angle;
                let x = self.radius * phi.cos();
                let z = self.radius * phi.sin();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        // Боковые грани
        for j in 0..h_segs {
            for i in 0..segs {
                let a = j * (segs + 1) + i;
                let b = a + 1;
                let c = (j + 1) * (segs + 1) + i;
                let d = c + 1;

                let color = self.get_face_color(j as f32 / h_segs as f32, i as f32 / segs as f32);
                faces.push(Face { vertices: vec![a, b, c], color });
                faces.push(Face { vertices: vec![b, d, c], color });
            }
        }

        // Верхняя и нижняя крышки
        let top_center = vertices.len();
        vertices.push(Vec3::new(0.0, self.height / 2.0, 0.0));
        let bottom_center = vertices.len();
        vertices.push(Vec3::new(0.0, -self.height / 2.0, 0.0));

        // Верхняя крышка
        let top_start = (h_segs) * (segs + 1);
        for i in 0..segs {
            let a = top_start + i;
            let b = top_start + i + 1;
            faces.push(Face {
                vertices: vec![top_center, a, b],
                color: egui::Color32::from_rgb(200, 200, 255),
            });
        }

        // Нижняя крышка
        let bottom_start = 0;
        for i in 0..segs {
            let a = bottom_start + i;
            let b = bottom_start + i + 1;
            faces.push(Face {
                vertices: vec![bottom_center, b, a],
                color: egui::Color32::from_rgb(200, 200, 255),
            });
        }

        self.vertices = vertices;
        self.faces = faces;
        self.apply_transform();
    }

    // --- Конус ---
    fn generate_cone(&mut self) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let segs = self.segments;
        let h_segs = self.height_segments;

        // Боковые вершины
        for j in 0..=h_segs {
            let t = j as f32 / h_segs as f32;
            let y = -self.height / 2.0 + t * self.height;
            let r = self.radius * (1.0 - t);

            for i in 0..=segs {
                let phi = (i as f32 / segs as f32) * 2.0 * PI;
                let x = r * phi.cos();
                let z = r * phi.sin();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        // Боковые грани
        for j in 0..h_segs {
            for i in 0..segs {
                let a = j * (segs + 1) + i;
                let b = a + 1;
                let c = (j + 1) * (segs + 1) + i;
                let d = c + 1;

                let color = self.get_face_color(j as f32 / h_segs as f32, i as f32 / segs as f32);
                faces.push(Face { vertices: vec![a, b, c], color });
                faces.push(Face { vertices: vec![b, d, c], color });
            }
        }

        // Нижняя крышка
        let bottom_center = vertices.len();
        vertices.push(Vec3::new(0.0, -self.height / 2.0, 0.0));

        let bottom_start = 0;
        for i in 0..segs {
            let a = bottom_start + i;
            let b = bottom_start + i + 1;
            faces.push(Face {
                vertices: vec![bottom_center, b, a],
                color: egui::Color32::from_rgb(200, 200, 255),
            });
        }

        self.vertices = vertices;
        self.faces = faces;
        self.apply_transform();
    }

    // --- Тор ---
    fn generate_torus(&mut self) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let segs = self.segments;
        let h_segs = self.height_segments;
        let r_major = self.radius;
        let r_minor = self.radius * 0.4;

        for j in 0..=h_segs {
            let theta = (j as f32 / h_segs as f32) * 2.0 * PI;
            let y = r_minor * theta.sin();

            for i in 0..=segs {
                let phi = (i as f32 / segs as f32) * 2.0 * PI;
                let x = (r_major + r_minor * theta.cos()) * phi.cos();
                let z = (r_major + r_minor * theta.cos()) * phi.sin();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        for j in 0..h_segs {
            for i in 0..segs {
                let a = j * (segs + 1) + i;
                let b = a + 1;
                let c = (j + 1) * (segs + 1) + i;
                let d = c + 1;

                let color = self.get_face_color(j as f32 / h_segs as f32, i as f32 / segs as f32);
                faces.push(Face { vertices: vec![a, b, c], color });
                faces.push(Face { vertices: vec![b, d, c], color });
            }
        }

        self.vertices = vertices;
        self.faces = faces;
        self.apply_transform();
    }

    // --- Пирамида ---
    fn generate_pyramid(&mut self) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let sides = self.segments.max(3);
        let apex = Vec3::new(0.0, self.height / 2.0, 0.0);

        // Вершины основания
        for i in 0..sides {
            let phi = (i as f32 / sides as f32) * 2.0 * PI;
            let x = self.radius * phi.cos();
            let z = self.radius * phi.sin();
            vertices.push(Vec3::new(x, -self.height / 2.0, z));
        }

        let base_center = vertices.len();
        vertices.push(Vec3::new(0.0, -self.height / 2.0, 0.0));
        let apex_idx = vertices.len();
        vertices.push(apex);

        // Боковые грани
        for i in 0..sides {
            let a = i;
            let b = (i + 1) % sides;
            let color = self.get_face_color(i as f32 / sides as f32, 0.5);
            faces.push(Face {
                vertices: vec![a, b, apex_idx],
                color,
            });
        }

        // Основание
        for i in 0..sides {
            let a = i;
            let b = (i + 1) % sides;
            faces.push(Face {
                vertices: vec![base_center, b, a],
                color: egui::Color32::from_rgb(150, 150, 200),
            });
        }

        self.vertices = vertices;
        self.faces = faces;
        self.apply_transform();
    }

    // --- Призма ---
    fn generate_prism(&mut self) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let sides = self.segments.max(3);

        // Верхние и нижние вершины
        for j in 0..2 {
            let y = if j == 0 { -self.height / 2.0 } else { self.height / 2.0 };
            for i in 0..sides {
                let phi = (i as f32 / sides as f32) * 2.0 * PI;
                let x = self.radius * phi.cos();
                let z = self.radius * phi.sin();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        let top_start = 0;
        let bottom_start = sides;

        // Боковые грани
        for i in 0..sides {
            let a = top_start + i;
            let b = top_start + (i + 1) % sides;
            let c = bottom_start + i;
            let d = bottom_start + (i + 1) % sides;

            let color = self.get_face_color(i as f32 / sides as f32, 0.5);
            faces.push(Face { vertices: vec![a, b, c], color });
            faces.push(Face { vertices: vec![b, d, c], color });
        }

        // Верхняя крышка
        let top_center = vertices.len();
        vertices.push(Vec3::new(0.0, self.height / 2.0, 0.0));
        for i in 0..sides {
            let a = top_start + i;
            let b = top_start + (i + 1) % sides;
            faces.push(Face {
                vertices: vec![top_center, a, b],
                color: egui::Color32::from_rgb(200, 200, 255),
            });
        }

        // Нижняя крышка
        let bottom_center = vertices.len();
        vertices.push(Vec3::new(0.0, -self.height / 2.0, 0.0));
        for i in 0..sides {
            let a = bottom_start + i;
            let b = bottom_start + (i + 1) % sides;
            faces.push(Face {
                vertices: vec![bottom_center, b, a],
                color: egui::Color32::from_rgb(200, 200, 255),
            });
        }

        self.vertices = vertices;
        self.faces = faces;
        self.apply_transform();
    }

    // --- Геодезическая (икосаэдр с подразбиением) ---
    fn generate_geodesic(&mut self) {
        // Начинаем с икосаэдра
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let mut vertices = vec![
            Vec3::new(-1.0, phi, 0.0).scale(self.radius),
            Vec3::new(1.0, phi, 0.0).scale(self.radius),
            Vec3::new(-1.0, -phi, 0.0).scale(self.radius),
            Vec3::new(1.0, -phi, 0.0).scale(self.radius),
            Vec3::new(0.0, -1.0, phi).scale(self.radius),
            Vec3::new(0.0, 1.0, phi).scale(self.radius),
            Vec3::new(0.0, -1.0, -phi).scale(self.radius),
            Vec3::new(0.0, 1.0, -phi).scale(self.radius),
            Vec3::new(phi, 0.0, -1.0).scale(self.radius),
            Vec3::new(phi, 0.0, 1.0).scale(self.radius),
            Vec3::new(-phi, 0.0, -1.0).scale(self.radius),
            Vec3::new(-phi, 0.0, 1.0).scale(self.radius),
        ];

        let mut faces = vec![
            [0, 1, 5], [0, 5, 11], [0, 11, 10], [0, 10, 7], [0, 7, 1],
            [1, 7, 8], [1, 8, 9], [1, 9, 5], [5, 9, 4], [5, 4, 11],
            [11, 4, 2], [11, 2, 10], [10, 2, 6], [10, 6, 7], [7, 6, 8],
            [8, 6, 3], [8, 3, 9], [9, 3, 4], [4, 3, 2], [2, 3, 6],
        ];

        // Подразбиваем грани (subdivision)
        let subdivisions = self.height_segments.min(3);
        for _ in 0..subdivisions {
            let (new_vertices, new_faces) = Self::subdivide_mesh(&vertices, &faces);
            vertices = new_vertices;
            faces = new_faces;
        }

        // Нормализуем вершины на сферу
        for v in &mut vertices {
            let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
            if len > 0.0 {
                *v = v.scale(self.radius / len);
            }
        }

        self.vertices = vertices;
        self.faces = faces.iter().map(|tri| {
            Face {
                vertices: vec![tri[0], tri[1], tri[2]],
                color: egui::Color32::from_rgb(
                    (100 + (tri[0] * 37 + tri[1] * 71 + tri[2] * 13) % 155) as u8,
                    (100 + (tri[0] * 53 + tri[1] * 29 + tri[2] * 97) % 155) as u8,
                    (100 + (tri[0] * 83 + tri[1] * 47 + tri[2] * 61) % 155) as u8,
                ),
            }
        }).collect();

        self.apply_transform();
    }

    // --- Вспомогательные функции ---

    fn subdivide_mesh(vertices: &[Vec3], faces: &[[usize; 3]]) -> (Vec<Vec3>, Vec<[usize; 3]>) {
        let mut new_vertices = vertices.to_vec();
        let mut new_faces = Vec::new();

        for &tri in faces {
            let a = tri[0];
            let b = tri[1];
            let c = tri[2];

            // Создаём новые вершины в серединах рёбер
            let ab = Self::midpoint(&new_vertices[a], &new_vertices[b]);
            let bc = Self::midpoint(&new_vertices[b], &new_vertices[c]);
            let ca = Self::midpoint(&new_vertices[c], &new_vertices[a]);

            let ab_idx = new_vertices.len();
            new_vertices.push(ab);
            let bc_idx = new_vertices.len();
            new_vertices.push(bc);
            let ca_idx = new_vertices.len();
            new_vertices.push(ca);

            // 4 новых треугольника
            new_faces.push([a, ab_idx, ca_idx]);
            new_faces.push([b, bc_idx, ab_idx]);
            new_faces.push([c, ca_idx, bc_idx]);
            new_faces.push([ab_idx, bc_idx, ca_idx]);
        }

        (new_vertices, new_faces)
    }

    fn midpoint(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0)
    }

    // --- Применение трансформаций ---

    fn apply_transform(&mut self) {
        for v in &mut self.vertices {
            *v = v.scale(self.scale_x).scale(self.scale_y).scale(self.scale_z);
        }
    }

    // --- Цветовые режимы ---

    fn get_face_color(&self, u: f32, v: f32) -> egui::Color32 {
        match self.color_mode {
            ColorMode::Solid => egui::Color32::from_rgb(100, 200, 255),
            ColorMode::Rainbow => {
                let hue = (u * 360.0 + v * 120.0) % 360.0;
                Self::hsl_to_color(hue, 0.8, 0.6)
            }
            ColorMode::Height => {
                let brightness = (u * 200.0 + 55.0) as u8;
                egui::Color32::from_rgb(brightness, brightness / 2, 255 - brightness / 2)
            }
            ColorMode::Random => {
                let r = ((u * 255.0) as u8) ^ ((v * 255.0) as u8);
                let g = ((u * 127.0 + v * 128.0) as u8) ^ 0x55;
                let b = ((u * 64.0 + v * 191.0) as u8) ^ 0xAA;
                egui::Color32::from_rgb(r, g, b)
            }
        }
    }

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

    // --- Отрисовка ---

    fn draw_3d_scene(&self, painter: &egui::Painter, rect: egui::Rect) {
        let center = rect.center();

        // Поворот
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

        // Проекция
        let projected: Vec<egui::Pos2> = rotated_vertices.iter()
            .map(|v| {
                let p = v.project(self.fov, self.distance);
                egui::pos2(center.x + p.x, center.y + p.y)
            })
            .collect();

        // Сортировка по глубине
        let mut sorted_faces: Vec<(usize, f32)> = self.faces.iter()
            .enumerate()
            .map(|(i, face)| {
                let avg_z = face.vertices.iter()
                    .map(|&idx| rotated_vertices[idx].z)
                    .sum::<f32>() / face.vertices.len() as f32;
                (i, avg_z)
            })
            .collect();
        sorted_faces.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Рисуем грани
        for (face_idx, _) in sorted_faces {
            let face = &self.faces[face_idx];
            let pts: Vec<egui::Pos2> = face.vertices.iter()
                .map(|&idx| projected[idx])
                .collect();

            if pts.len() >= 3 {
                if !self.show_wireframe {
                    let mut color = face.color;
                    if self.smooth_shading {
                        // Простая имитация освещения
                        let brightness = 0.5 + 0.5 * (face_idx as f32 / self.faces.len() as f32);
                        color = egui::Color32::from_rgb(
                            (color.r() as f32 * brightness) as u8,
                            (color.g() as f32 * brightness) as u8,
                            (color.b() as f32 * brightness) as u8,
                        );
                    }
                    let fill_shape = egui::Shape::convex_polygon(
                        pts.clone(),
                        color,
                        egui::Stroke::NONE,
                    );
                    painter.add(fill_shape);
                }

                // Обводка
                for i in 0..pts.len() {
                    let next = (i + 1) % pts.len();
                    painter.line_segment(
                        [pts[i], pts[next]],
                        egui::Stroke::new(1.0, egui::Color32::WHITE),
                    );
                }
            }
        }

        // Вершины
        if self.show_vertices {
            for (i, &pos) in projected.iter().enumerate() {
                painter.circle_filled(pos, 3.0, egui::Color32::YELLOW);
                if projected.len() < 50 {
                    painter.text(
                        pos + egui::vec2(5.0, -5.0),
                        egui::Align2::LEFT_BOTTOM,
                        format!("{}", i),
                        egui::FontId::proportional(8.0),
                        egui::Color32::WHITE,
                    );
                }
            }
        }

        // Информация
        painter.text(
            egui::pos2(rect.left() + 10.0, rect.top() + 10.0),
            egui::Align2::LEFT_TOP,
            format!("Вершин: {}, Граней: {}", self.vertices.len(), self.faces.len()),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }
}

impl eframe::App for ParametricApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_rotating {
            self.angle_x += self.speed_x;
            self.angle_y += self.speed_y;
            self.angle_z += self.speed_z;
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Заголовок
            ui.heading("🎨 3D Параметрический конструктор");
            ui.separator();

            // Основная область с графикой и панелями
            ui.columns(2, |columns| {
                // Левая колонка - 3D сцена
                columns[0].with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    let (rect_response, painter) = ui.allocate_painter(
                        ui.available_size(),
                        egui::Sense::drag()
                    );
                    let rect = rect_response.rect;

                    // Фон
                    painter.rect_filled(
                        rect,
                        0.0,
                        egui::Color32::from_rgb(10, 10, 30),
                    );

                    // Сетка
                    for i in 0..20 {
                        let pos = i as f32 / 20.0;
                        let x = rect.left() + rect.width() * pos;
                        let y = rect.top() + rect.height() * pos;
                        painter.line_segment(
                            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                            egui::Stroke::new(0.3, egui::Color32::from_rgba_premultiplied(50, 50, 100, 20)),
                        );
                        painter.line_segment(
                            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                            egui::Stroke::new(0.3, egui::Color32::from_rgba_premultiplied(50, 50, 100, 20)),
                        );
                    }

                    // 3D сцена
                    self.draw_3d_scene(&painter, rect);
                });

                // Правая колонка - панель управления
                columns[1].with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.heading("⚙️ Параметры");
                    ui.separator();

                    // --- Вращение ---
                    ui.collapsing("🔄 Вращение", |ui| {
                        ui.horizontal(|ui| {
                            if ui.button(if self.is_rotating { "⏸" } else { "▶" }).clicked() {
                                self.is_rotating = !self.is_rotating;
                            }
                            if ui.button("🔄 Сброс").clicked() {
                                self.angle_x = 0.0;
                                self.angle_y = 0.0;
                                self.angle_z = 0.0;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::Slider::new(&mut self.speed_x, 0.0..=0.02).text(""));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Y:");
                            ui.add(egui::Slider::new(&mut self.speed_y, 0.0..=0.02).text(""));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Z:");
                            ui.add(egui::Slider::new(&mut self.speed_z, 0.0..=0.02).text(""));
                        });
                    });

                    // --- Форма ---
                    ui.collapsing("🔷 Форма", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Тип:");
                            egui::ComboBox::from_label("")
                                .selected_text(self.primitive_type.name())
                                .show_ui(ui, |ui| {
                                    for &primitive in &[
                                        PrimitiveType::Sphere,
                                        PrimitiveType::Cylinder,
                                        PrimitiveType::Cone,
                                        PrimitiveType::Torus,
                                        PrimitiveType::Pyramid,
                                        PrimitiveType::Prism,
                                        PrimitiveType::Geodesic,
                                    ] {
                                        if ui.selectable_label(self.primitive_type == primitive, primitive.name()).clicked() {
                                            self.primitive_type = primitive;
                                            self.generate_mesh();
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Сегменты:");
                            ui.add(egui::Slider::new(&mut self.segments, 3..=64)
                                .text(""))
                                .on_hover_text("Количество сегментов по окружности");
                            if ui.button("🔄").clicked() {
                                self.generate_mesh();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Высота:");
                            ui.add(egui::Slider::new(&mut self.height_segments, 1..=32)
                                .text(""))
                                .on_hover_text("Количество сегментов по высоте");
                            if ui.button("🔄").clicked() {
                                self.generate_mesh();
                            }
                        });

                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Радиус:");
                            ui.add(egui::Slider::new(&mut self.radius, 20.0..=200.0).text(""));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Высота:");
                            ui.add(egui::Slider::new(&mut self.height, 50.0..=300.0).text(""));
                        });

                        if self.primitive_type == PrimitiveType::Cylinder {
                            ui.horizontal(|ui| {
                                ui.label("Скручивание:");
                                ui.add(egui::Slider::new(&mut self.twist, 0.0..=PI * 2.0).text(""));
                            });
                        }

                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Масштаб X:");
                            ui.add(egui::Slider::new(&mut self.scale_x, 0.5..=2.0).text(""));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Масштаб Y:");
                            ui.add(egui::Slider::new(&mut self.scale_y, 0.5..=2.0).text(""));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Масштаб Z:");
                            ui.add(egui::Slider::new(&mut self.scale_z, 0.5..=2.0).text(""));
                        });

                        if ui.button("🔄 Применить трансформации").clicked() {
                            self.apply_transform();
                        }
                    });

                    // --- Визуализация ---
                    ui.collapsing("🎨 Визуализация", |ui| {
                        ui.checkbox(&mut self.show_vertices, "Показывать вершины");
                        ui.checkbox(&mut self.show_wireframe, "Каркасный режим");
                        ui.checkbox(&mut self.show_normals, "Показывать нормали");
                        ui.checkbox(&mut self.smooth_shading, "Сглаженное освещение");

                        ui.separator();

                        ui.label("Цветовой режим:");
                        egui::ComboBox::from_label("")
                            .selected_text(match self.color_mode {
                                ColorMode::Solid => "Сплошной",
                                ColorMode::Rainbow => "Радуга",
                                ColorMode::Height => "По высоте",
                                ColorMode::Random => "Случайный",
                            })
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(self.color_mode == ColorMode::Solid, "Сплошной").clicked() {
                                    self.color_mode = ColorMode::Solid;
                                    self.generate_mesh();
                                }
                                if ui.selectable_label(self.color_mode == ColorMode::Rainbow, "Радуга").clicked() {
                                    self.color_mode = ColorMode::Rainbow;
                                    self.generate_mesh();
                                }
                                if ui.selectable_label(self.color_mode == ColorMode::Height, "По высоте").clicked() {
                                    self.color_mode = ColorMode::Height;
                                    self.generate_mesh();
                                }
                                if ui.selectable_label(self.color_mode == ColorMode::Random, "Случайный").clicked() {
                                    self.color_mode = ColorMode::Random;
                                    self.generate_mesh();
                                }
                            });
                    });

                    // --- Камера ---
                    ui.collapsing("📷 Камера", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("FOV:");
                            ui.add(egui::Slider::new(&mut self.fov, 100.0..=600.0).text(""));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Дистанция:");
                            ui.add(egui::Slider::new(&mut self.distance, 200.0..=1000.0).text(""));
                        });
                    });

                    // --- Информация ---
                    ui.separator();
                    ui.label(format!("Вершин: {}", self.vertices.len()));
                    ui.label(format!("Граней: {}", self.faces.len()));

                    ui.separator();

                    if ui.button("❌ Выход").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    }
}