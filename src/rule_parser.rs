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
    // TODO: instead, maybe create a regex and match on it
    let option_length: usize = option.len();

    match option_length {
        4 => {
            None

        },
        3 => {
            None
        },
        _ => None
    }
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
