use eframe::egui;
use chrono::Local;

// Подключаем модуль календаря
mod calendar;
use calendar::DatePicker;

// ============================================================================
// МОДЕЛИ ДАННЫХ
// ============================================================================

/// Список доступных предметов
const SUBJECTS: &[&str] = &[
    "Русский язык",
    "Математика",
    "Физика",
    "Химия",
    "Биология",
    "География",
    "Обществознание",
    "История",
    "Литература",
    "Информатика",
    "Английский язык",
    "Немецкий язык",
    "Французский язык",
    "Испанский язык",
];

/// Тип экзамена
#[derive(Clone, Copy, Debug, PartialEq)]
enum ExamType {
    Ege,
    Oge,
}

impl ExamType {
    fn as_str(&self) -> &str {
        match self {
            ExamType::Ege => "ЕГЭ",
            ExamType::Oge => "ОГЭ",
        }
    }

    fn max_score(&self, subject: &str) -> u32 {
        match self {
            ExamType::Ege => 100,
            ExamType::Oge => {
                match subject.to_lowercase().as_str() {
                    "русский язык" | "русский" => 37,
                    "математика" => 31,
                    "физика" => 39,
                    "химия" => 38,
                    "биология" => 47,
                    "география" => 31,
                    "обществознание" => 37,
                    "история" => 37,
                    "литература" => 42,
                    "информатика" | "икт" => 21,
                    "английский язык" | "немецкий язык" | "французский язык" |
                    "испанский язык" | "английский" | "немецкий" |
                    "французский" | "испанский" => 68,
                    _ => 50, // Дефолтное значение для неизвестных предметов
                }
            }
        }
    }
}

/// Профиль предмета
#[derive(Clone, Debug)]
struct SubjectProfile {
    id: usize,
    name: String,              // Название предмета
    exam_type: ExamType,       // ЕГЭ или ОГЭ
    target_score: u32,         // Целевой балл (0-100)
    daily_time: u32,           // Время в день (минуты)
    entries: Vec<StudyEntry>,  // История записей
}

/// Запись о решенных задачах
#[derive(Clone, Debug)]
struct StudyEntry {
    date: String,
    task_type: String,
    solved: u32,
    correct: u32,
}

// ============================================================================
// ЭКРАНЫ ПРИЛОЖЕНИЯ
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
enum Screen {
    ProfileSelection,
    ProfileCreation,
    StudyTracker(usize), // ID активного профиля
}

// ============================================================================
// ГЛАВНОЕ ПРИЛОЖЕНИЕ
// ============================================================================

struct StudyTrackerApp {
    // Состояние навигации
    current_screen: Screen,

    // Данные
    profiles: Vec<SubjectProfile>,
    next_profile_id: usize,

    // Буферы для создания профиля
    new_profile_name: String,
    new_profile_exam: ExamType,
    new_profile_target: u32,
    new_profile_time: u32,

    // Буферы для добавления записи
    input_date: String,
    input_type: String,
    input_solved: u32,
    input_correct: u32,
}

impl Default for StudyTrackerApp {
    fn default() -> Self {
        Self {
            current_screen: Screen::ProfileSelection,
            profiles: Vec::new(),
            next_profile_id: 0,

            new_profile_name: SUBJECTS[0].to_string(), // Русский язык по умолчанию
            new_profile_exam: ExamType::Ege,
            new_profile_target: 80,
            new_profile_time: 60,

            input_date: Local::now().format("%d.%m.%Y").to_string(),
            input_type: "Тип 1".to_owned(),
            input_solved: 0,
            input_correct: 0,
        }
    }
}

impl StudyTrackerApp {
    fn create_profile(&mut self) {
        let profile = SubjectProfile {
            id: self.next_profile_id,
            name: self.new_profile_name.clone(),
            exam_type: self.new_profile_exam,
            target_score: self.new_profile_target,
            daily_time: self.new_profile_time,
            entries: Vec::new(),
        };

        self.profiles.push(profile);
        self.next_profile_id += 1;

        // Сброс на первый предмет из списка
        self.new_profile_name = SUBJECTS[0].to_string();
        self.new_profile_target = 80;
        self.new_profile_time = 60;

        // Возврат к выбору профилей
        self.current_screen = Screen::ProfileSelection;
    }

    fn get_active_profile_mut(&mut self, id: usize) -> Option<&mut SubjectProfile> {
        self.profiles.iter_mut().find(|p| p.id == id)
    }
}

impl eframe::App for StudyTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        match self.current_screen {
            Screen::ProfileSelection => self.show_profile_selection(ctx),
            Screen::ProfileCreation => self.show_profile_creation(ctx),
            Screen::StudyTracker(profile_id) => self.show_study_tracker(ctx, profile_id),
        }
    }
}

// ============================================================================
// ЭКРАН: ВЫБОР ПРОФИЛЯ
// ============================================================================

impl StudyTrackerApp {
    fn show_profile_selection(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(40.0);

            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("📚 Трекер прогресса").size(32.0));
                ui.add_space(10.0);
                ui.label("Выберите предмет или создайте новый профиль");
            });

            ui.add_space(30.0);

            // Сетка профилей
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    let card_width = 300.0;

                    for profile in &self.profiles {
                        let frame = egui::Frame::group(ui.style())
                            .inner_margin(egui::Margin::same(15.0))
                            .fill(egui::Color32::from_gray(30));

                        frame.show(ui, |ui| {
                            ui.set_min_width(card_width);

                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&profile.name)
                                        .size(20.0)
                                        .strong());
                                    ui.label(format!("Цель: {} баллов · {}",
                                                     profile.target_score,
                                                     profile.exam_type.as_str()));
                                    ui.label(format!("⏰ {} мин/день", profile.daily_time));
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button(egui::RichText::new("Открыть →").size(14.0)).clicked() {
                                        self.current_screen = Screen::StudyTracker(profile.id);
                                    }
                                });
                            });
                        });

                        ui.add_space(10.0);
                    }

                    // Кнопка создания профиля
                    ui.add_space(10.0);

                    let btn = egui::Button::new(
                        egui::RichText::new("➕ Добавить профиль").size(16.0)
                    )
                        .fill(egui::Color32::from_rgb(0, 150, 100))
                        .min_size(egui::vec2(card_width, 50.0));

                    if ui.add(btn).clicked() {
                        self.current_screen = Screen::ProfileCreation;
                    }
                });
            });
        });
    }
}

// ============================================================================
// ЭКРАН: СОЗДАНИЕ ПРОФИЛЯ
// ============================================================================

impl StudyTrackerApp {
    fn show_profile_creation(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(40.0);

            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Создание профиля").size(28.0));
                ui.add_space(30.0);
            });

            // Форма создания
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.set_max_width(500.0);

                    // Выпадающее меню для выбора предмета
                    ui.label("Предмет");
                    egui::ComboBox::from_id_source("subject_select")
                        .selected_text(&self.new_profile_name)
                        .width(300.0)
                        .show_ui(ui, |ui| {
                            for subject in SUBJECTS {
                                ui.selectable_value(&mut self.new_profile_name, subject.to_string(), *subject);
                            }
                        });
                    ui.add_space(15.0);

                    // Тип экзамена
                    ui.label("Тип экзамена");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.new_profile_exam, ExamType::Ege, "ЕГЭ");
                        ui.selectable_value(&mut self.new_profile_exam, ExamType::Oge, "ОГЭ");
                    });
                    ui.add_space(15.0);

                    // Вычисляем максимальный балл на основе экзамена и предмета
                    let max_score = self.new_profile_exam.max_score(&self.new_profile_name);

                    // Ограничиваем текущий балл максимумом
                    if self.new_profile_target > max_score {
                        self.new_profile_target = max_score;
                    }

                    // Целевой балл - простой ввод числа
                    ui.label(format!("Целевой балл (макс: {})", max_score));
                    ui.add(egui::DragValue::new(&mut self.new_profile_target)
                        .clamp_range(0..=max_score)
                        .speed(1));

                    // Показываем подсказку для ОГЭ
                    if self.new_profile_exam == ExamType::Oge {
                        ui.label(
                            egui::RichText::new(format!("ℹ Макс. балл для этого предмета: {}", max_score))
                                .small()
                                .color(egui::Color32::GRAY)
                        );
                    }
                    ui.add_space(15.0);

                    // Время в день - простой ввод числа без ограничений
                    ui.label("Время в день (минуты)");
                    ui.add(egui::DragValue::new(&mut self.new_profile_time)
                        .clamp_range(0..=u32::MAX)
                        .speed(5));
                    ui.add_space(25.0);

                    // Кнопки
                    ui.horizontal(|ui| {
                        if ui.button("← Назад").clicked() {
                            self.current_screen = Screen::ProfileSelection;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui::RichText::new("Создать").size(16.0)).clicked() {
                                self.create_profile();
                            }
                        });
                    });
                });
        });
    }
}

// ============================================================================
// ЭКРАН: ТРЕКЕР УЧЕБЫ
// ============================================================================

impl StudyTrackerApp {
    fn show_study_tracker(&mut self, ctx: &egui::Context, profile_id: usize) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Кнопка возврата
            if ui.button("← К профилям").clicked() {
                self.current_screen = Screen::ProfileSelection;
            }

            ui.add_space(10.0);

            // Получаем профиль (используем временное заимствование)
            let profile_name = self.profiles.iter()
                .find(|p| p.id == profile_id)
                .map(|p| p.name.clone())
                .unwrap_or_default();

            let profile_exam = self.profiles.iter()
                .find(|p| p.id == profile_id)
                .map(|p| p.exam_type)
                .unwrap_or(ExamType::Ege);

            // Заголовок
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new(&profile_name).size(24.0));
                ui.label(egui::RichText::new(profile_exam.as_str()).color(egui::Color32::GRAY));
            });

            ui.separator();

            // Панель ввода
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Дата");
                        ui.text_edit_singleline(&mut self.input_date);
                    });

                    ui.vertical(|ui| {
                        ui.label("Тип задания");
                        ui.text_edit_singleline(&mut self.input_type);
                    });

                    ui.vertical(|ui| {
                        ui.label("Решено");
                        ui.add(egui::DragValue::new(&mut self.input_solved));
                    });

                    ui.vertical(|ui| {
                        ui.label("Верно");
                        ui.add(egui::DragValue::new(&mut self.input_correct));
                    });

                    ui.add_space(10.0);

                    let btn = egui::Button::new("Добавить")
                        .fill(egui::Color32::from_rgb(0, 150, 100));

                    if ui.add_sized([80.0, 30.0], btn).clicked() {
                        // Клонируем данные ПЕРЕД мутабельным заимствованием
                        let new_entry = StudyEntry {
                            date: self.input_date.clone(),
                            task_type: self.input_type.clone(),
                            solved: self.input_solved,
                            correct: self.input_correct,
                        };

                        if let Some(profile) = self.get_active_profile_mut(profile_id) {
                            profile.entries.push(new_entry);
                        }
                    }
                });
            });

            ui.add_space(20.0);

            // Список записей
            if let Some(profile) = self.profiles.iter().find(|p| p.id == profile_id) {
                ui.heading(format!("Сегодня · {}", self.input_date));
                ui.separator();

                for (idx, entry) in profile.entries.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("#{}", idx + 1)).strong());
                            ui.label(&entry.task_type);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("✓ {}", entry.correct));
                                ui.label(format!("📝 {}", entry.solved));
                            });
                        });
                    });
                    ui.add_space(5.0);
                }
            }
        });
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Study Tracker",
        options,
        Box::new(|_cc| Box::new(StudyTrackerApp::default())),
    )
}