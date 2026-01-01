use eframe::egui;
use chrono::{Local, NaiveDate, Datelike};

/// Виджет календаря для выбора даты
pub struct DatePicker {
    pub selected_date: NaiveDate,
    pub display_month: NaiveDate,
}

impl DatePicker {
    pub fn new(initial_date: NaiveDate) -> Self {
        Self {
            selected_date: initial_date,
            display_month: initial_date,
        }
    }

    /// Показывает календарь и возвращает true если дата изменилась
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut date_changed = false;
        let today = Local::now().date_naive();

        ui.set_min_width(280.0);

        // Навигация по месяцам
        ui.horizontal(|ui| {
            if ui.button("◀").clicked() {
                self.display_month = self.display_month
                    .checked_sub_months(chrono::Months::new(1))
                    .unwrap_or(self.display_month);
            }

            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                let month_name = match self.display_month.month() {
                    1 => "Январь", 2 => "Февраль", 3 => "Март", 4 => "Апрель",
                    5 => "Май", 6 => "Июнь", 7 => "Июль", 8 => "Август",
                    9 => "Сентябрь", 10 => "Октябрь", 11 => "Ноябрь", 12 => "Декабрь",
                    _ => "?",
                };

                ui.label(egui::RichText::new(
                    format!("{} {}", month_name, self.display_month.year())
                ).strong());
            });

            let can_go_forward = self.display_month.year() < today.year()
                || (self.display_month.year() == today.year() && self.display_month.month() < today.month());

            ui.add_enabled_ui(can_go_forward, |ui| {
                if ui.button("▶").clicked() {
                    self.display_month = self.display_month
                        .checked_add_months(chrono::Months::new(1))
                        .unwrap_or(self.display_month);
                }
            });
        });

        ui.separator();

        // Сетка календаря
        egui::Grid::new("calendar_grid")
            .spacing([2.0, 2.0])
            .show(ui, |ui| {
                for day in &["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"] {
                    ui.label(egui::RichText::new(*day).strong().small());
                }
                ui.end_row();

                let first_day = NaiveDate::from_ymd_opt(
                    self.display_month.year(),
                    self.display_month.month(),
                    1
                ).unwrap();

                let first_weekday = first_day.weekday().num_days_from_monday() as usize;

                let days_in_month = if self.display_month.month() == 12 {
                    NaiveDate::from_ymd_opt(self.display_month.year() + 1, 1, 1)
                        .unwrap()
                        .signed_duration_since(first_day)
                        .num_days()
                } else {
                    NaiveDate::from_ymd_opt(self.display_month.year(), self.display_month.month() + 1, 1)
                        .unwrap()
                        .signed_duration_since(first_day)
                        .num_days()
                };

                for _ in 0..first_weekday {
                    ui.label("");
                }

                for day in 1..=days_in_month {
                    let date = NaiveDate::from_ymd_opt(
                        self.display_month.year(),
                        self.display_month.month(),
                        day as u32
                    ).unwrap();

                    let is_selected = date == self.selected_date;
                    let is_today = date == today;
                    let is_future = date > today;

                    let mut button = egui::Button::new(format!("{}", day))
                        .min_size(egui::vec2(30.0, 25.0));

                    if is_selected {
                        button = button.fill(egui::Color32::from_rgb(0, 150, 100));
                    } else if is_today {
                        button = button.fill(egui::Color32::from_rgb(50, 50, 100));
                    }

                    ui.add_enabled_ui(!is_future, |ui| {
                        if ui.add(button).clicked() {
                            self.selected_date = date;
                            date_changed = true;
                        }
                    });

                    if (first_weekday + day as usize - 1) % 7 == 6 {
                        ui.end_row();
                    }
                }
            });

        date_changed
    }
}