use api::{Rule, WeekdayOption, Recur, Weekday};
use regex::{Regex, Captures};
use chrono::NaiveDateTime;

/**
 * Takes a string such as: `[2018-01-01 00:00 a]: New Years`
 */
pub fn parse(line: &String) -> Option<Rule> {
    let schedule_re: Regex = Regex::new(r"\[(.*)\](.*)")
        .expect("to be a regex rule");

    let caps: Captures =
        match schedule_re.captures(line) {
            None => return None,
            Some(caps) => caps
        };
    assert_eq!(caps.len(), 2, "Schedule should have format [YYYY-MM-DD hh:mm [recurring [recurring_opts]] description");

    let schedule_parts: Vec<&str> = caps[0].split(" ").collect();

    let epoch_date: NaiveDateTime =
        match schedule_parts.get(0) {
            Some(e) => {
                NaiveDateTime::parse_from_str(e, "%Y-%m-%d %H:%M")
                    .expect("DateTime to have format %Y-%m-%d %H:%M")
            },
            None => return None
        };

    let recurring: Option<Recur> =
        match schedule_parts.get(1) {
            Some(e) => parse_recur(e),
            None => None
        };

    let options: Vec<WeekdayOption> =
        match schedule_parts.get(2) {
            Some(e) => parse_weekdays(&e.to_string()),
            None => vec![]
        };

    let description: String = caps[1].to_string();

    Some(Rule {
        epoch_date: epoch_date,
        recurring: recurring,
        options: options,
        description: description
    })
}

/**
 * Takes a string such as: "mon", "4thu", "1wed,4fri", "mon,tue,wed"
 */
pub fn parse_weekdays(options: &String) -> Vec<WeekdayOption> {
    if options.len() == 0 {
        return vec![];
    }

    options
        .split(",")
        .fold(vec![], |mut acc, e| {
            if let Some(weekday_option) = parse_weekday(&e.to_owned()) {
                acc.push(weekday_option);
            }
            acc
        })
}

/**
 * Takes a string such as: "mon", "4thu"
 */
pub fn parse_weekday(option: &String) -> Option<WeekdayOption> {
    let offset: u8;
    let dow: String;
    let re = Regex::new(r"\d(mon|tue|wed|thu|fri|sat|sun)")
        .expect("regex should be valid");

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
pub fn parse_recur(option: &str) -> Option<Recur> {
    match option {
        "a" => Some(Recur::ANNUALLY),
        "m" => Some(Recur::MONTHLY),
        "w" => Some(Recur::WEEKLY),
        "d" => Some(Recur::DAILY),
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
mod parse_weekdays_test {
    use rule;
    use api::{WeekdayOption, Weekday};

    #[test]
    fn none_when_empty() {
        let res = rule::parse_weekdays(&String::from(""));
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn one_option() {
        let res: Vec<WeekdayOption> = rule::parse_weekdays(&String::from("wed"));
        assert_eq!(res[0].weekday, Weekday::WED);
    }

    #[test]
    fn many_options() {
        let res: Vec<WeekdayOption> = rule::parse_weekdays(&String::from("2wed,thu"));
        assert_eq!(res[0].weekday, Weekday::WED);
        assert_eq!(res[0].offset, 2);
        assert_eq!(res[1].weekday, Weekday::THU);
        assert_eq!(res[1].offset, 0);
    }
}

#[cfg(test)]
mod parse_weekday_test {
    use rule;
    use api::{WeekdayOption, Weekday};

    #[test]
    fn parse_weekday_with_option() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("4mon"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::MON);
        assert_eq!(res.offset, 4);
    }

    #[test]
    fn parse_weekday_no_option() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("mon"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::MON);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn parse_tue() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("tue"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::TUE);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn parse_wed() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("wed"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::WED);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn parse_thu() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("thu"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::THU);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn parse_fri() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("fri"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::FRI);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn parse_sat() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("sat"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::SAT);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn parse_sun() {
        let res: WeekdayOption = rule::parse_weekday(&String::from("sun"))
            .expect("to find a weekday option");
        assert_eq!(res.weekday, Weekday::SUN);
        assert_eq!(res.offset, 0);
    }

    #[test]
    fn none_for_garbage() {
        let res = rule::parse_weekday(&String::from("asd"));
        assert_eq!(res.is_none(), true);
    }

    #[test]
    fn none_for_garbage_with_offset() {
        let res = rule::parse_weekday(&String::from("4asd"));
        assert_eq!(res.is_none(), true);
    }
}


#[cfg(test)]
mod parse_recur_test {
    use rule;
    use api::Recur;

    #[test]
    fn gets_annually() {
        let res: Recur = rule::parse_recur("a").unwrap();
        assert_eq!(res, Recur::ANNUALLY);
    }

    #[test]
    fn gets_monthly() {
        let res: Recur = rule::parse_recur("m").unwrap();
        assert_eq!(res, Recur::MONTHLY);
    }

    #[test]
    fn gets_weekly() {
        let res: Recur = rule::parse_recur("w").unwrap();
        assert_eq!(res, Recur::WEEKLY);
    }

    #[test]
    fn gets_daily() {
        let option: String = "d".to_owned();
        let res: Recur = rule::parse_recur("d").unwrap();
        assert_eq!(res, Recur::DAILY);
    }
}
