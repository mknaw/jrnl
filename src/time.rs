use std::fmt;

/// Week days

#[derive(Copy, Clone, PartialEq)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub const WEEKDAYS: [WeekDay; 7] = [
    WeekDay::Monday,
    WeekDay::Tuesday,
    WeekDay::Wednesday,
    WeekDay::Thursday,
    WeekDay::Friday,
    WeekDay::Saturday,
    WeekDay::Sunday,
];

impl fmt::Display for WeekDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Monday => "Mon",
                Self::Tuesday => "Tue",
                Self::Wednesday => "Wed",
                Self::Thursday => "Thu",
                Self::Friday => "Fri",
                Self::Saturday => "Sat",
                Self::Sunday => "Sun",
            }
        )
    }
}

impl WeekDay {
    // TODO probably should be fmt::Display
    pub fn nday(&self) -> usize {
        match self {
            Self::Monday => 0,
            Self::Tuesday => 1,
            Self::Wednesday => 2,
            Self::Thursday => 3,
            Self::Friday => 4,
            Self::Saturday => 5,
            Self::Sunday => 6,
        }
    }
}

/// Months

#[derive(Copy, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

pub const MONTHS: [Month; 12] = [
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::January => "Jan",
                Self::February => "Feb",
                Self::March => "Mar",
                Self::April => "Apr",
                Self::May => "May",
                Self::June => "Jun",
                Self::July => "Jul",
                Self::August => "Aug",
                Self::September => "Sep",
                Self::October => "Oct",
                Self::November => "Nov",
                Self::December => "Dec",
            }
        )
    }
}

impl Month {
    pub fn weekday_offset_from_jan(&self) -> u8 {
        // TODO doing all this on u8s instead of a week day abstraction is ugly
        match self {
            Self::January | Self::October => 0,
            Self::February | Self::March | Self::November => 3,
            Self::April | Self::July => 6,
            Self::May => 1,
            Self::June => 4,
            Self::August => 2,
            Self::September | Self::December => 5,
        }
    }

    pub fn leap_year_weekday_offset_from_jan(&self) -> u8 {
        match self {
            Self::January | Self::February => self.weekday_offset_from_jan(),
            _ => self.weekday_offset_from_jan() + 1,
        }
    }
}

/// Year
pub struct Year {
    year: u32,
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.year)
    }
}

impl Year {
    pub fn is_leap(&self) -> bool {
        if self.year % 100 == 0 {
            return self.year % 400 == 0;
        }
        return self.year % 4 == 0;
    }
}

pub struct MonthYear {
    pub month: Month,
    pub year: Year,
}

impl fmt::Display for MonthYear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.month, self.year)
    }
}

impl MonthYear {
    pub fn new(month: Month, year: u32) -> Self {
        let year = Year { year: year };
        Self {
            month: month,
            year: year,
        }
    }

    pub fn n_days(&self) -> u8 {
        match self.month {
            Month::February if self.year.is_leap() => 29,
            Month::February => 28,

            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => 31,

            Month::April | Month::June | Month::September | Month::November => 30,
        }
    }

    fn jan_first_weekday(&self) -> u8 {
        // Gauss' algo for calculating weekday of 1/1 for any given year.
        let year = self.year.year;
        let mut nday = 1;
        nday += 5 * ((year - 1) % 4);
        nday += 4 * ((year - 1) % 100);
        nday += 6 * ((year - 1) % 400);
        nday %= 7;
        (nday - 1) as u8
    }

    pub fn first_weekday(&self) -> WeekDay {
        let jan_first_weekday = self.jan_first_weekday();
        if self.year.is_leap() {
            let n_weekday =
                (jan_first_weekday + self.month.leap_year_weekday_offset_from_jan()) % 7;
            return WEEKDAYS[n_weekday as usize];
        } else {
            let n_weekday = (jan_first_weekday + self.month.weekday_offset_from_jan()) % 7;
            return WEEKDAYS[n_weekday as usize];
        }
    }
}
