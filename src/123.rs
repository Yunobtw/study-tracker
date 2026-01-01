// use eframe::egui;
// use serde::{Deserialize, Serialize};

// // --- 1. СТРУКТУРЫ ДАННЫХ ---

// // Описываем одну запись (строчку в таблице)
// #[derive(Clone, Serialize, Deserialize)]
// struct StudyEntry {
//     date: String,
//     subject: String,
//     task_type: String,
//     solved: u32,
//     correct: u32,
// }

// // Описываем состояние всего приложения
// #[derive(Serialize, Deserialize)]
// struct StudyTrackerApp {
//     // Вектор (список) всех наших записей
//     entries: Vec<StudyEntry>,

//     // Поля ввода (их не нужно сохранять в файл, поэтому skip)
//     #[serde(skip)]
//     input_date: String,
//     #[serde(skip)]
//     input_subject: String,
//     #[serde(skip)]
//     input_type: String,
//     #[serde(skip)]
//     input_solved: u32,
//     #[serde(skip)]
//     input_correct: u32,
// }

// // Значения по умолчанию (при первом запуске)
// impl Default for StudyTrackerApp {
//     fn default() -> Self {
//         Self {
//             entries: Vec::new(),
//             input_date: chrono::Local::now().format("%d.%m.%Y").to_string(),
//             input_subject: "Информатика".to_owned(),
//             input_type: "Тип 1".to_owned(),
//             input_solved: 0,
//             input_correct: 0,
//         }
//     }
// }

// // --- 2. ЛОГИКА ПРИЛОЖЕНИЯ ---

// impl StudyTrackerApp {
//     // Функция для настройки шрифтов (чтобы работала кириллица)
//     fn configure_fonts(ctx: &egui::Context) {
//         let mut fonts = egui::FontDefinitions::default();

//         // Добавляем поддержку кириллицы
//         fonts.font_data.insert(
//             "my_font".to_owned(),
//             egui::FontData::from_static(include_bytes!("c:/Windows/Fonts/arial.ttf")),
//             // ВНИМАНИЕ: Если вы на Linux/Mac, путь будет другой!
//             // Если Arial нет, код упадет. Ниже я напишу как сделать универсально.
//         );

//         // Ставим этот шрифт основным
//         fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
//             .insert(0, "my_font".to_owned());

//         fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
//             .push("my_font".to_owned());

//         ctx.set_fonts(fonts);
//     }
// }

// impl eframe::App for StudyTrackerApp {
//     // Эта функция отвечает за СОХРАНЕНИЕ состояния
//     fn save(&mut self, storage: &mut dyn eframe::Storage) {
//         eframe::set_value(storage, eframe::APP_KEY, self);
//     }

//     // Эта функция вызывается каждый кадр для отрисовки (UPDATE)
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         // Устанавливаем темную тему
//         ctx.set_visuals(egui::Visuals::dark());

//         egui::CentralPanel::default().show(ctx, |ui| {
//             // --- ВЕРХНЯЯ ПАНЕЛЬ: ВВОД ДАННЫХ ---
//             ui.push_id("input_area", |ui| {
//                 egui::Frame::group(ui.style())
//                     .inner_margin(10.0)
//                     .show(ui, |ui| {

//                         ui.horizontal(|ui| {
//                             // Поле Даты
//                             ui.vertical(|ui| {
//                                 ui.label("Дата");
//                                 ui.add(egui::TextEdit::singleline(&mut self.input_date).desired_width(80.0));
//                             });

//                             // Выбор Предмета
//                             ui.vertical(|ui| {
//                                 ui.label("Предмет");
//                                 egui::ComboBox::from_id_source("subj")
//                                     .selected_text(&self.input_subject)
//                                     .show_ui(ui, |ui| {
//                                         ui.selectable_value(&mut self.input_subject, "Информатика".to_string(), "Информатика");
//                                         ui.selectable_value(&mut self.input_subject, "Математика".to_string(), "Математика");
//                                         ui.selectable_value(&mut self.input_subject, "Физика".to_string(), "Физика");
//                                     });
//                             });

//                             // Выбор Типа
//                             ui.vertical(|ui| {
//                                 ui.label("Тип");
//                                 egui::ComboBox::from_id_source("type")
//                                     .selected_text(&self.input_type)
//                                     .show_ui(ui, |ui| {
//                                         ui.selectable_value(&mut self.input_type, "Тип 1".to_string(), "Тип 1");
//                                         ui.selectable_value(&mut self.input_type, "Тип 13".to_string(), "Тип 13");
//                                         ui.selectable_value(&mut self.input_type, "Тип 24".to_string(), "Тип 24");
//                                     });
//                             });

//                             // Числовые поля
//                             ui.vertical(|ui| {
//                                 ui.label("Решено");
//                                 ui.add(egui::DragValue::new(&mut self.input_solved));
//                             });
//                             ui.vertical(|ui| {
//                                 ui.label("Верно");
//                                 ui.add(egui::DragValue::new(&mut self.input_correct));
//                             });

//                             ui.add_space(10.0);

//                             // КНОПКА ДОБАВИТЬ
//                             let btn = egui::Button::new("Добавить")
//                                 .fill(egui::Color32::from_rgb(0, 168, 107)) // Зеленый цвет
//                                 .min_size(egui::vec2(80.0, 30.0));

//                             if ui.add(btn).clicked() {
//                                 // Логика добавления записи в вектор
//                                 self.entries.push(StudyEntry {
//                                     date: self.input_date.clone(),
//                                     subject: self.input_subject.clone(),
//                                     task_type: self.input_type.clone(),
//                                     solved: self.input_solved,
//                                     correct: self.input_correct,
//                                 });
//                             }
//                         });
//                     });
//             });

//             ui.add_space(20.0);

//             // --- ОСНОВНАЯ ЧАСТЬ (2 КОЛОНКИ) ---
//             ui.columns(2, |columns| {

//                 // ЛЕВАЯ КОЛОНКА: СПИСОК
//                 columns[0].vertical(|ui| {
//                     ui.heading(format!("Сегодня · {}", self.input_date));
//                     ui.separator();

//                     egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
//                         // .rev() чтобы новые были сверху
//                         for (i, entry) in self.entries.iter().rev().enumerate() {
//                             ui.group(|ui| {
//                                 ui.horizontal(|ui| {
//                                     ui.label(egui::RichText::new(format!("#{}", self.entries.len() - i)).strong());
//                                     ui.label(format!("{} | {}", entry.subject, entry.task_type));

//                                     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
//                                         // Расчет процента
//                                         let percent = if entry.solved > 0 {
//                                             (entry.correct as f32 / entry.solved as f32 * 100.0) as i32
//                                         } else { 0 };

//                                         let color = if percent > 80 { egui::Color32::GREEN } else { egui::Color32::YELLOW };
//                                         ui.colored_label(color, format!("{}%", percent));
//                                         ui.label(format!("{}/{}", entry.correct, entry.solved));
//                                     });
//                                 });
//                             });
//                             ui.add_space(4.0);
//                         }
//                     });
//                 });

//                 // ПРАВАЯ КОЛОНКА: СТАТИСТИКА
//                 columns[1].vertical(|ui| {
//                     ui.heading("Статистика (Информатика)");
//                     ui.separator();

//                     // Пример статических данных (позже можно сделать расчет)
//                     draw_progress_bar(ui, "Всего", 0.83, "83% · 33/40");
//                     ui.add_space(10.0);
//                     draw_progress_bar(ui, "За неделю", 0.60, "60% · 20/33");

//                     ui.add_space(20.0);
//                     ui.label(egui::RichText::new("По типам").strong());
//                     draw_progress_bar(ui, "Тип 1", 0.75, "75%");
//                     draw_progress_bar(ui, "Тип 13", 0.90, "90%");
//                 });
//             });

//             ui.add_space(30.0);
//             ui.separator();

//             // --- НИЖНЯЯ ЧАСТЬ: HEATMAP (GITHUB STYLE) ---
//             ui.heading("Активность");
//             ui.add_space(5.0);
//             draw_heatmap(ui);
//         });
//     }
// }

// // Вспомогательная функция для рисования прогресс-бара
// fn draw_progress_bar(ui: &mut egui::Ui, label: &str, progress: f32, text: &str) {
//     ui.label(label);
//     let bar = egui::ProgressBar::new(progress)
//         .text(text)
//         .fill(egui::Color32::from_rgb(50, 100, 255)); // Синий цвет
//     ui.add(bar);
// }

// // Вспомогательная функция для рисования "Зеленых квадратиков"
// fn draw_heatmap(ui: &mut egui::Ui) {
//     let (rect, _resp) = ui.allocate_at_least(egui::vec2(500.0, 100.0), egui::Sense::hover());

//     // Если есть место для рисования
//     if ui.is_rect_visible(rect) {
//         let painter = ui.painter();
//         let start_pos = rect.min;
//         let box_size = 12.0;
//         let gap = 3.0;

//         // Рисуем сетку 52 недели x 7 дней
//         for week in 0..52 {
//             for day in 0..7 {
//                 let x = start_pos.x + (week as f32 * (box_size + gap));
//                 let y = start_pos.y + (day as f32 * (box_size + gap));

//                 // Тут можно добавить логику: если в этот день были записи -> зеленый
//                 // Пока сделаем рандомный узор для красоты
//                 let intensity = ((week + day) % 5) as f32 / 5.0;

//                 let color = if intensity > 0.6 {
//                     egui::Color32::from_rgb(0, (intensity * 200.0) as u8, 0) // Оттенки зеленого
//                 } else {
//                     egui::Color32::from_rgb(40, 40, 40) // Серый (нет активности)
//                 };

//                 painter.rect_filled(
//                     egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(box_size, box_size)),
//                     2.0, // скругление углов
//                     color
//                 );
//             }
//         }
//     }
// }

// // --- 3. ТОЧКА ВХОДА ---

// fn main() -> eframe::Result<()> {
//     // Настройки окна
//     let native_options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_inner_size([900.0, 600.0]), // Размер окна
//         ..Default::default()
//     };

//     eframe::run_native(
//         "Study Tracker",
//         native_options,
//         // Загрузка приложения
//         Box::new(|cc| {
//             // 1. Настраиваем шрифты (чтобы был русский)
//             StudyTrackerApp::configure_fonts(&cc.egui_ctx);

//             // 2. Пытаемся загрузить старые данные из файла
//             if let Some(storage) = cc.storage {
//                 if let Some(app) = eframe::get_value::<StudyTrackerApp>(storage, eframe::APP_KEY) {
//                     return Box::new(app);
//                 }
//             }

//             // 3. Если данных нет, создаем новое приложение
//             Box::new(StudyTrackerApp::default())
//         }),
//     )
// }