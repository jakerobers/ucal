use api::{Rule, WeekdayOption, Recur, Weekday};
use regex::Regex;

/**
 * Takes a string such as: `[2018-01-01 00:00 a] New Years`
 */
pub fn parse(_line: &String) -> Option<Rule> {
    None
}

/**
 * Takes a string such as: "mon", "4thu", "1wed,4fri", "mon,tue,wed"
 */
pub fn parse_weekdays(_option: &String) -> Option<Vec<WeekdayOption>> {
    None
}

/**
 * Takes a string such as: "mon", "4thu"
 */
pub fn parse_weekday(option: &String) -> Option<WeekdayOption> {
    let re = Regex::new(r"\d(mon|tue|wed|thu|fri|sat|sun)")
        .expect("regex should be valid");

    let offset: u8;
    let dow: String;

    if re.is_match(option) {
        let first: String = option.chars().take(1).collect();
        offset = first.parse::<u8>().expect("offset should be a number");
        dow = option.chars().skip(1).take(3).collect();
    } else {
        offset = 0;
        dow = option.to_owned();
    }

    let dow: Option<Weekday> =
        match dow {
            _ if dow == "mon" => Some(Weekday::MON),
            _ if dow == "tue" => Some(Weekday::TUE),
            _ if dow == "wed" => Some(Weekday::WED),
            _ if dow == "thu" => Some(Weekday::THU),
            _ if dow == "fri" => Some(Weekday::FRI),
            _ if dow == "sat" => Some(Weekday::SAT),
            _ if dow == "sun" => Some(Weekday::SUN),
            _ => None
        };

    if let None = dow {
        return None;
    }

    let dow: Weekday = dow.unwrap();
    let weekday_option = WeekdayOption {
        offset: offset,
        weekday: dow
    };
    Some(weekday_option)
}

/**
 * Takes a string such as: "a", "m", "w", "d"
 */
pub fn parse_recur(option: &String) -> Option<Recur> {
    match option {
        _ if "a" == option => Some(Recur::ANNUALLY),
        _ if "m" == option => Some(Recur::MONTHLY),
        _ if "w" == option => Some(Recur::WEEKLY),
        _ if "d" == option => Some(Recur::DAILY),
        _ => None
    }
}

#[cfg(test)]
mod parse_test {
    #[test]
    fn returns_some_value() {
    }

    #[test]
    fn returns_none_for_invalid() {
    }

    #[test]
    fn returns_first_set() {
    }
}

#[cfg(test)]
mod get_weekdays_test {

}

#[cfg(test)]
mod get_recur_test {
    use rule_parser;
    use api::Recur;

    #[test]
    fn gets_annually() {
        let option: String = "a".to_owned();
        let res: Recur = rule_parser::parse_recur(&option).unwrap();
        assert_eq!(res, Recur::ANNUALLY);
    }

    #[test]
    fn gets_monthly() {
        let option: String = "m".to_owned();
        let res: Recur = rule_parser::parse_recur(&option).unwrap();
        assert_eq!(res, Recur::MONTHLY);
    }

    #[test]
    fn gets_weekly() {
        let option: String = "w".to_owned();
        let res: Recur = rule_parser::parse_recur(&option).unwrap();
        assert_eq!(res, Recur::WEEKLY);
    }

    #[test]
    fn gets_daily() {
        let option: String = "d".to_owned();
        let res: Recur = rule_parser::parse_recur(&option).unwrap();
        assert_eq!(res, Recur::DAILY);
    }
}
