use cursive::direction::Orientation;
use cursive::vec::Vec2;
use cursive::views::{Dialog, LinearLayout};
use cursive::{Printer, View};

mod theme;
mod time;

struct MonthView {
    size: Vec2,
    month_year: time::MonthYear,
}

impl MonthView {
    pub fn new(month_year: time::MonthYear) -> Self {
        Self {
            size: (31, 8).into(),
            month_year: month_year,
        }
    }
}

impl View for MonthView {
    fn draw(&self, printer: &Printer) {
        printer.print((14, 1), self.month_year.month.to_string().as_mut_str());

        let mut n_week_day = self.month_year.first_weekday().nday();
        let mut x = 4 * n_week_day + 2;
        let mut y = 2;

        for _ in 0..self.month_year.n_days() {
            if n_week_day > 6 {
                n_week_day = 0;
                x = 2;
                y += 1;
            }

            printer.print((x, y), time::WEEKDAYS[n_week_day].to_string().as_mut_str());
            x += 4;
            n_week_day += 1;
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size
    }
}

fn get_layout() -> LinearLayout {
    let mut row_layout = LinearLayout::new(Orientation::Vertical);

    for month_chunk in time::MONTHS.chunks(4) {
        let mut col_layout = LinearLayout::new(Orientation::Horizontal);
        for month in month_chunk.iter() {
            col_layout.add_child(MonthView::new(time::MonthYear::new(*month, 2020)));
        }
        row_layout.add_child(col_layout);
    }
    row_layout
}

fn main() {
    let mut cur = cursive::default();

    cur.set_theme(theme::get());

    cur.add_global_callback('q', |c| c.quit());

    cur.add_layer(Dialog::around(get_layout()));
    cur.run();
}
