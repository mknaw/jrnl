use cursive::{
    direction::Orientation,
    event::{Event, EventResult, Key},
    vec::Vec2,
    view::Selector,
    views::{DummyView, LinearLayout},
    {Printer, View},
    traits::*,
};

mod theme;
mod time;

struct MonthView {
    size: Vec2,
    month_year: time::MonthYear,
    focus: u8,
}

impl MonthView {
    pub fn new(month_year: time::MonthYear) -> Self {
        Self {
            size: (28, 6).into(),
            month_year: month_year,
            focus: 0,
        }
    }

    fn inc_day(&mut self) {
        if self.focus < self.month_year.n_days() - 1 {
            self.focus += 1;
        }
    }

    fn dec_day(&mut self) {
        if self.focus > 0 {
            self.focus -= 1;
        }
    }

    fn inc_week(&mut self) {
        if self.focus < self.month_year.n_days() - 8 {
            self.focus += 7;
        }
    }

    fn dec_week(&mut self) {
        if self.focus >= 7 {
            self.focus -= 7;
        }
    }
}

impl View for MonthView {
    fn draw(&self, printer: &Printer) {
        printer.print((12, 0), self.month_year.month.to_string().as_mut_str());

        let mut n_week_day = self.month_year.first_weekday().nday();
        let mut x = 4 * n_week_day;
        let mut y = 1;

        for i in 0..self.month_year.n_days() {
            if n_week_day >= 7 {
                n_week_day = 0;
                x = 0;
                y += 1;
            }
            printer.focused(self.month_year.to_string() == "Jan2020").with_selection(i == self.focus, |printer| {
                printer.print((x, y), time::WEEKDAYS[n_week_day].to_string().as_mut_str());
            });
            x += 4;
            n_week_day += 1;
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) => self.dec_week(),
            Event::Key(Key::Down) => self.inc_week(),
            Event::Key(Key::Left) => self.dec_day(),
            Event::Key(Key::Right) => self.inc_day(),
            _ => {}
        }
        EventResult::Consumed(None)
    }
}

fn get_layout() -> LinearLayout {
    let mut row_layout = LinearLayout::new(Orientation::Vertical);

    for month_chunk in time::MONTHS.chunks(4) {
        let mut col_layout = LinearLayout::new(Orientation::Horizontal);
        for month in month_chunk.iter() {
            let monthyear = time::MonthYear::new(*month, 2020);
            let name = monthyear.to_string();
            col_layout.add_child(MonthView::new(monthyear).with_name(name));
            col_layout.add_child(DummyView);
        }
        row_layout.add_child(col_layout);
        row_layout.add_child(DummyView);
    }
    row_layout
}

fn main() {
    let mut cur = cursive::default();

    cur.set_theme(theme::get());

    cur.add_global_callback('q', |c| c.quit());

    let mut layout = get_layout();
    layout.focus_view(&Selector::Name("Aug2020")).unwrap();
    cur.add_layer(layout);
    // cur.call_on_name("Aug2020", |view: &mut MonthView| {
        // view.enabled = true;
    // });
    // cur.add_layer(MonthView::new(time::MonthYear::new(time::Month::August, 2020)));
    cur.run();
}
