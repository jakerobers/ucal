use chrono::NaiveDateTime;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Recur {
    ANNUALLY,
    MONTHLY,
    WEEKLY,
    DAILY,
}

#[derive(Debug)]
pub enum Weekday {
    MON,
    TUE,
    WED,
    THU,
    FRI,
    SAT,
    SUN,
}

#[derive(Debug)]
pub struct WeekdayOption {
    pub offset: u8,
    pub weekday: Weekday,
}

#[derive(Debug)]
pub struct Rule {
    pub epoch_date: NaiveDateTime,
    pub recurring: Recur,
    pub options: Vec<WeekdayOption>,
    pub description: String,
}
