const MIN_TAG: usize = 1;
const MAX_TAG: usize = 255;
const MIN_WORKFLOW_NAME: usize = 1;
const MAX_WORKFLOW_NAME: usize = 500;
const SECONDS_CEILING: u32 = 31_557_600;
const MINUTES_CEILING: u32 = 525_960;
const HOURS_CEILING: u32 = 8_766;
const DAYS_CEILING: u32 = 365;
const WEEKS_CEILING: u32 = 52;
const MONTHS_CEILING: u32 = 12;
const LAST_HOUR: u8 = 23;
const LAST_MINUTE: u8 = 59;
const LAST_MONTH_DAY: i8 = 31;
const LAST_DAY_OF_MONTH: i8 = -1;
const HOURLY_FOREVER: u32 = 131_400;
const DAILY_FOREVER: u32 = 5_475;
const WEEKLY_FOREVER: u32 = 780;
const MONTHLY_FOREVER: u32 = 180;

macro_rules! rejected_error {
    ($error:ident, $noun:expr, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, Debug)]
        pub struct $error {
            rejected: String,
        }

        impl std::fmt::Display for $error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "'{}' is not {}", self.rejected, $noun)
            }
        }

        impl std::error::Error for $error {}
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Delay {
    count: u32,
    unit: DelayUnit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DelayUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
}

impl DelayUnit {
    fn suffix(self) -> &'static str {
        match self {
            DelayUnit::Seconds => "s",
            DelayUnit::Minutes => "m",
            DelayUnit::Hours => "h",
            DelayUnit::Days => "d",
            DelayUnit::Weeks => "w",
            DelayUnit::Months => "mo",
        }
    }

    fn ceiling(self) -> u32 {
        match self {
            DelayUnit::Seconds => SECONDS_CEILING,
            DelayUnit::Minutes => MINUTES_CEILING,
            DelayUnit::Hours => HOURS_CEILING,
            DelayUnit::Days => DAYS_CEILING,
            DelayUnit::Weeks => WEEKS_CEILING,
            DelayUnit::Months => MONTHS_CEILING,
        }
    }

    fn parse_suffix(suffix: &str) -> Option<DelayUnit> {
        match suffix {
            "m" => Some(DelayUnit::Minutes),
            "h" => Some(DelayUnit::Hours),
            "d" => Some(DelayUnit::Days),
            "w" => Some(DelayUnit::Weeks),
            "mo" => Some(DelayUnit::Months),
            _ => None,
        }
    }
}

impl std::str::FromStr for Delay {
    type Err = DelayError;

    fn from_str(text: &str) -> Result<Delay, DelayError> {
        let rejected = || DelayError {
            rejected: text.to_string(),
        };
        let digits = text
            .find(|character: char| !character.is_ascii_digit())
            .ok_or_else(rejected)?;
        let unit = DelayUnit::parse_suffix(&text[digits..]).ok_or_else(rejected)?;
        let count: u32 = text[..digits].parse().map_err(|_| rejected())?;
        if count < 1 || count > unit.ceiling() {
            return Err(rejected());
        }
        Ok(Delay { count, unit })
    }
}

impl std::convert::TryFrom<&str> for Delay {
    type Error = DelayError;

    fn try_from(text: &str) -> Result<Delay, DelayError> {
        text.parse()
    }
}

impl std::fmt::Display for Delay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.count, self.unit.suffix())
    }
}

impl serde::Serialize for Delay {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let count = self.count;
        let duration = match self.unit {
            DelayUnit::Seconds => format!("PT{count}S"),
            DelayUnit::Minutes => format!("PT{count}M"),
            DelayUnit::Hours => format!("PT{count}H"),
            DelayUnit::Days => format!("P{count}D"),
            DelayUnit::Weeks => format!("P{count}W"),
            DelayUnit::Months => format!("P{count}M"),
        };
        serializer.serialize_str(&duration)
    }
}

impl<'de> serde::Deserialize<'de> for Delay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Delay, D::Error> {
        fn single(text: &str) -> Option<(DelayUnit, u32)> {
            let rest = text.strip_prefix('P')?;
            let (timed, rest) = match rest.strip_prefix('T') {
                Some(rest) => (true, rest),
                None => (false, rest),
            };
            let digits = rest.find(|character: char| !character.is_ascii_digit())?;
            let count: u32 = rest[..digits].parse().ok()?;
            let unit = match (timed, rest[digits..].chars().next()?) {
                (true, 'S') => DelayUnit::Seconds,
                (true, 'M') => DelayUnit::Minutes,
                (true, 'H') => DelayUnit::Hours,
                (false, 'D') => DelayUnit::Days,
                (false, 'W') => DelayUnit::Weeks,
                (false, 'M') => DelayUnit::Months,
                _ => return None,
            };
            Some((unit, count))
        }

        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        match single(&text) {
            Some((unit, count)) if count > 0 => Ok(Delay { count, unit }),
            _ => Err(serde::de::Error::custom(DelayError { rejected: text })),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendDays {
    days: std::collections::BTreeSet<Weekday>,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

const WEEK: [Weekday; 7] = [
    Weekday::Sunday,
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
];

const WEEKDAYS: [Weekday; 5] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
];

impl Weekday {
    fn word(self) -> &'static str {
        match self {
            Weekday::Sunday => "sun",
            Weekday::Monday => "mon",
            Weekday::Tuesday => "tue",
            Weekday::Wednesday => "wed",
            Weekday::Thursday => "thu",
            Weekday::Friday => "fri",
            Weekday::Saturday => "sat",
        }
    }

    fn parse_word(word: &str) -> Option<Weekday> {
        WEEK.into_iter().find(|day| day.word() == word)
    }
}

impl std::str::FromStr for SendDays {
    type Err = SendDaysError;

    fn from_str(text: &str) -> Result<SendDays, SendDaysError> {
        let rejected = || SendDaysError {
            rejected: text.to_string(),
        };
        let collapsed = match text {
            "weekdays" => Some(WEEKDAYS.to_vec()),
            "every-day" => Some(WEEK.to_vec()),
            _ => None,
        };
        if let Some(days) = collapsed {
            return Ok(SendDays {
                days: days.into_iter().collect(),
            });
        }
        let mut days = std::collections::BTreeSet::new();
        for word in text.split(',') {
            let day = Weekday::parse_word(word).ok_or_else(rejected)?;
            if !days.insert(day) {
                return Err(rejected());
            }
        }
        if days.is_empty() {
            return Err(rejected());
        }
        Ok(SendDays { days })
    }
}

impl std::convert::TryFrom<&str> for SendDays {
    type Error = SendDaysError;

    fn try_from(text: &str) -> Result<SendDays, SendDaysError> {
        text.parse()
    }
}

impl std::fmt::Display for SendDays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words: Vec<&str> = self.days.iter().map(|day| day.word()).collect();
        f.write_str(&words.join(","))
    }
}

impl SendDays {
    fn rule_text(&self) -> String {
        let days: Vec<&str> = self
            .days
            .iter()
            .map(|day| match day {
                Weekday::Sunday => "SU",
                Weekday::Monday => "MO",
                Weekday::Tuesday => "TU",
                Weekday::Wednesday => "WE",
                Weekday::Thursday => "TH",
                Weekday::Friday => "FR",
                Weekday::Saturday => "SA",
            })
            .collect();
        format!("BYDAY={}", days.join(","))
    }

    fn from_rule(rule: &str) -> Option<SendDays> {
        let listed = rule_part(rule, "BYDAY")?;
        let mut days = std::collections::BTreeSet::new();
        for word in listed.split(',') {
            let day = match word {
                "SU" => Weekday::Sunday,
                "MO" => Weekday::Monday,
                "TU" => Weekday::Tuesday,
                "WE" => Weekday::Wednesday,
                "TH" => Weekday::Thursday,
                "FR" => Weekday::Friday,
                "SA" => Weekday::Saturday,
                _ => return None,
            };
            days.insert(day);
        }
        if days.is_empty() {
            return None;
        }
        Some(SendDays { days })
    }
}

fn rule_part<'a>(rule: &'a str, key: &str) -> Option<&'a str> {
    rule.split(';')
        .filter_map(|entry| entry.split_once('='))
        .find(|(name, _)| *name == key)
        .map(|(_, part)| part)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SendTime {
    hour: u8,
    minute: u8,
}

impl std::str::FromStr for SendTime {
    type Err = SendTimeError;

    fn from_str(text: &str) -> Result<SendTime, SendTimeError> {
        let rejected = || SendTimeError {
            rejected: text.to_string(),
        };
        let (hours, minutes) = text.split_once(':').ok_or_else(rejected)?;
        if hours.len() != 2 || minutes.len() != 2 {
            return Err(rejected());
        }
        let hour: u8 = hours.parse().map_err(|_| rejected())?;
        let minute: u8 = minutes.parse().map_err(|_| rejected())?;
        clocked(hour, minute).ok_or_else(rejected)
    }
}

fn clocked(hour: u8, minute: u8) -> Option<SendTime> {
    if hour > LAST_HOUR || minute > LAST_MINUTE {
        return None;
    }
    Some(SendTime { hour, minute })
}

impl std::convert::TryFrom<&str> for SendTime {
    type Error = SendTimeError;

    fn try_from(text: &str) -> Result<SendTime, SendTimeError> {
        text.parse()
    }
}

impl std::fmt::Display for SendTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl SendTime {
    fn rule_text(&self) -> String {
        format!("BYHOUR={};BYMINUTE={}", self.hour, self.minute)
    }

    fn from_rule(rule: &str) -> Option<SendTime> {
        let hour: u8 = rule_part(rule, "BYHOUR")?.parse().ok()?;
        let minute: u8 = rule_part(rule, "BYMINUTE")?.parse().ok()?;
        clocked(hour, minute)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WaitTiming {
    pub delay: Option<Delay>,
    pub schedules: Vec<Recurrence>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimezoneSource {
    Workflow,
    Subscriber,
}

impl std::str::FromStr for TimezoneSource {
    type Err = TimezoneSourceError;

    fn from_str(text: &str) -> Result<TimezoneSource, TimezoneSourceError> {
        match text {
            "yes" => Ok(TimezoneSource::Subscriber),
            "no" => Ok(TimezoneSource::Workflow),
            _ => Err(TimezoneSourceError {
                rejected: text.to_string(),
            }),
        }
    }
}

impl std::convert::TryFrom<&str> for TimezoneSource {
    type Error = TimezoneSourceError;

    fn try_from(text: &str) -> Result<TimezoneSource, TimezoneSourceError> {
        text.parse()
    }
}

impl std::fmt::Display for TimezoneSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TimezoneSource::Workflow => "workflow",
            TimezoneSource::Subscriber => "subscriber",
        })
    }
}

impl serde::Serialize for TimezoneSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(matches!(self, TimezoneSource::Subscriber))
    }
}

impl<'de> serde::Deserialize<'de> for TimezoneSource {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<TimezoneSource, D::Error> {
        let subscriber = <bool as serde::Deserialize>::deserialize(deserializer)?;
        Ok(if subscriber {
            TimezoneSource::Subscriber
        } else {
            TimezoneSource::Workflow
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recurrence {
    frequency: Frequency,
    days: Option<SendDays>,
    day_of_month: Option<MonthDay>,
    at: Option<SendTime>,
    count: Option<u32>,
    until: Option<chrono::NaiveDate>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ends {
    Never,
    After { checks: u32 },
    Until { day: chrono::NaiveDate },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonthDay(i8);

impl std::fmt::Display for MonthDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            LAST_DAY_OF_MONTH => f.write_str("last"),
            day => write!(f, "{day}"),
        }
    }
}

impl Recurrence {
    pub fn daily(days: SendDays, at: SendTime) -> Recurrence {
        Recurrence {
            frequency: Frequency::Daily,
            days: Some(days),
            day_of_month: None,
            at: Some(at),
            count: None,
            until: None,
        }
    }

    pub fn ending(self, ends: Ends) -> Recurrence {
        let (count, until) = match ends {
            Ends::Never => (Some(self.frequency.forever()), None),
            Ends::After { checks } => (Some(checks), None),
            Ends::Until { day } => (None, Some(day)),
        };
        Recurrence {
            count,
            until,
            ..self
        }
    }

    pub fn with_days(self, days: SendDays) -> Recurrence {
        Recurrence {
            days: Some(days),
            ..self
        }
    }

    pub fn with_time(self, at: SendTime) -> Recurrence {
        Recurrence {
            at: Some(at),
            ..self
        }
    }

    pub fn days(&self) -> Option<&SendDays> {
        self.days.as_ref()
    }

    pub fn day_of_month(&self) -> Option<MonthDay> {
        self.day_of_month
    }

    pub fn at(&self) -> Option<SendTime> {
        self.at
    }

    pub fn ends(&self) -> Ends {
        if let Some(day) = self.until {
            return Ends::Until { day };
        }
        match self.count {
            Some(checks) if checks < self.frequency.forever() => Ends::After { checks },
            _ => Ends::Never,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Frequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl Frequency {
    fn interval(self) -> &'static str {
        match self {
            Frequency::Hourly => "1h",
            Frequency::Daily => "1d",
            Frequency::Weekly => "1w",
            Frequency::Monthly => "1mo",
        }
    }

    fn word(self) -> &'static str {
        match self {
            Frequency::Hourly => "HOURLY",
            Frequency::Daily => "DAILY",
            Frequency::Weekly => "WEEKLY",
            Frequency::Monthly => "MONTHLY",
        }
    }

    fn forever(self) -> u32 {
        match self {
            Frequency::Hourly => HOURLY_FOREVER,
            Frequency::Daily => DAILY_FOREVER,
            Frequency::Weekly => WEEKLY_FOREVER,
            Frequency::Monthly => MONTHLY_FOREVER,
        }
    }
}

const RULE_PARTS: [&str; 7] = [
    "FREQ",
    "BYDAY",
    "BYMONTHDAY",
    "BYHOUR",
    "BYMINUTE",
    "COUNT",
    "UNTIL",
];

const UNTIL_FORMAT: &str = "%Y%m%dT%H%M%SZ";

const FREQUENCIES: [Frequency; 4] = [
    Frequency::Hourly,
    Frequency::Daily,
    Frequency::Weekly,
    Frequency::Monthly,
];

impl std::str::FromStr for Recurrence {
    type Err = RecurrenceError;

    fn from_str(text: &str) -> Result<Recurrence, RecurrenceError> {
        let frequency = FREQUENCIES
            .into_iter()
            .find(|frequency| frequency.interval() == text)
            .ok_or_else(|| RecurrenceError {
                rejected: text.to_string(),
            })?;
        let (days, day_of_month, at) = match frequency {
            Frequency::Hourly => (None, None, None),
            Frequency::Daily => (
                Some("every-day".parse().expect("every-day is a day set")),
                None,
                clocked(8, 0),
            ),
            Frequency::Weekly => (
                Some("sun".parse().expect("sun is a day set")),
                None,
                clocked(8, 0),
            ),
            Frequency::Monthly => (None, Some(MonthDay(1)), clocked(8, 0)),
        };
        Ok(Recurrence {
            frequency,
            days,
            day_of_month,
            at,
            count: Some(frequency.forever()),
            until: None,
        })
    }
}

impl std::convert::TryFrom<&str> for Recurrence {
    type Error = RecurrenceError;

    fn try_from(text: &str) -> Result<Recurrence, RecurrenceError> {
        text.parse()
    }
}

impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.frequency.interval())
    }
}

impl serde::Serialize for Recurrence {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut parts = vec![format!("FREQ={}", self.frequency.word())];
        if let Some(days) = &self.days {
            parts.push(days.rule_text());
        }
        if let Some(day) = self.day_of_month {
            parts.push(format!("BYMONTHDAY={}", day.0));
        }
        if let Some(at) = self.at {
            parts.push(at.rule_text());
        }
        if let Some(count) = self.count {
            parts.push(format!("COUNT={count}"));
        }
        if let Some(day) = self.until {
            parts.push(format!(
                "UNTIL={}",
                day.and_hms_opt(0, 0, 0)
                    .unwrap_or_default()
                    .format(UNTIL_FORMAT)
            ));
        }
        serializer.serialize_str(&parts.join(";"))
    }
}

impl<'de> serde::Deserialize<'de> for Recurrence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Recurrence, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        let rejected = || {
            serde::de::Error::custom(RecurrenceError {
                rejected: text.clone(),
            })
        };
        let named = text
            .split(';')
            .filter_map(|entry| entry.split_once('='))
            .all(|(name, _)| RULE_PARTS.contains(&name));
        if !named {
            return Err(rejected());
        }
        let frequency = FREQUENCIES
            .into_iter()
            .find(|frequency| rule_part(&text, "FREQ") == Some(frequency.word()))
            .ok_or_else(rejected)?;
        let day_of_month = match rule_part(&text, "BYMONTHDAY") {
            None => None,
            Some(part) => {
                let day: i8 = part.parse().map_err(|_| rejected())?;
                if day != LAST_DAY_OF_MONTH && !(1..=LAST_MONTH_DAY).contains(&day) {
                    return Err(rejected());
                }
                Some(MonthDay(day))
            }
        };
        let count = match rule_part(&text, "COUNT") {
            None => None,
            Some(part) => Some(part.parse::<u32>().map_err(|_| rejected())?),
        };
        let until = match rule_part(&text, "UNTIL") {
            None => None,
            Some(part) => Some(
                chrono::NaiveDateTime::parse_from_str(part, UNTIL_FORMAT)
                    .map_err(|_| rejected())?
                    .date(),
            ),
        };
        Ok(Recurrence {
            frequency,
            days: SendDays::from_rule(&text),
            day_of_month,
            at: SendTime::from_rule(&text),
            count,
            until,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timezone(String);

impl Timezone {
    pub fn utc() -> Timezone {
        Timezone("UTC".to_string())
    }
}

impl std::str::FromStr for Timezone {
    type Err = TimezoneError;

    fn from_str(text: &str) -> Result<Timezone, TimezoneError> {
        let rejected = TimezoneError {
            rejected: text.to_string(),
        };
        if text.chars().any(char::is_whitespace) {
            return Err(rejected);
        }
        let mut parts = text.split('/');
        let area = parts.next().unwrap_or_default();
        let location = parts.next();
        if area.is_empty() || parts.next().is_some() {
            return Err(rejected);
        }
        match location {
            Some("") => Err(rejected),
            _ => Ok(Timezone(text.to_string())),
        }
    }
}

impl std::convert::TryFrom<&str> for Timezone {
    type Error = TimezoneError;

    fn try_from(text: &str) -> Result<Timezone, TimezoneError> {
        text.parse()
    }
}

impl std::fmt::Display for Timezone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for Timezone {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Timezone {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Timezone, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

fn linked(text: &str) -> bool {
    ["http://", "https://"].into_iter().any(|scheme| {
        text.strip_prefix(scheme)
            .is_some_and(|rest| !rest.is_empty())
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedUrl(String);

impl std::str::FromStr for FeedUrl {
    type Err = FeedUrlError;

    fn from_str(text: &str) -> Result<FeedUrl, FeedUrlError> {
        if linked(text) {
            Ok(FeedUrl(text.to_string()))
        } else {
            Err(FeedUrlError {
                rejected: text.to_string(),
            })
        }
    }
}

impl std::convert::TryFrom<&str> for FeedUrl {
    type Error = FeedUrlError;

    fn try_from(text: &str) -> Result<FeedUrl, FeedUrlError> {
        text.parse()
    }
}

impl std::fmt::Display for FeedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for FeedUrl {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for FeedUrl {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<FeedUrl, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkUrl(String);

impl std::str::FromStr for LinkUrl {
    type Err = LinkUrlError;

    fn from_str(text: &str) -> Result<LinkUrl, LinkUrlError> {
        if linked(text) {
            Ok(LinkUrl(text.to_string()))
        } else {
            Err(LinkUrlError {
                rejected: text.to_string(),
            })
        }
    }
}

impl std::convert::TryFrom<&str> for LinkUrl {
    type Error = LinkUrlError;

    fn try_from(text: &str) -> Result<LinkUrl, LinkUrlError> {
        text.parse()
    }
}

impl std::fmt::Display for LinkUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for LinkUrl {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for LinkUrl {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<LinkUrl, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkFragment(String);

impl std::str::FromStr for LinkFragment {
    type Err = LinkFragmentError;

    fn from_str(text: &str) -> Result<LinkFragment, LinkFragmentError> {
        if text.is_empty() {
            Err(LinkFragmentError {
                rejected: text.to_string(),
            })
        } else {
            Ok(LinkFragment(text.to_string()))
        }
    }
}

impl std::convert::TryFrom<&str> for LinkFragment {
    type Error = LinkFragmentError;

    fn try_from(text: &str) -> Result<LinkFragment, LinkFragmentError> {
        text.parse()
    }
}

impl std::fmt::Display for LinkFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for LinkFragment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for LinkFragment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<LinkFragment, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkMatch {
    Url(LinkUrl),
    Fragment(LinkFragment),
}

impl std::str::FromStr for LinkMatch {
    type Err = LinkMatchError;

    fn from_str(text: &str) -> Result<LinkMatch, LinkMatchError> {
        if linked(text) {
            return Ok(LinkMatch::Url(text.parse().map_err(
                |_: LinkUrlError| LinkMatchError {
                    rejected: text.to_string(),
                },
            )?));
        }
        text.parse()
            .map(LinkMatch::Fragment)
            .map_err(|_: LinkFragmentError| LinkMatchError {
                rejected: text.to_string(),
            })
    }
}

impl std::convert::TryFrom<&str> for LinkMatch {
    type Error = LinkMatchError;

    fn try_from(text: &str) -> Result<LinkMatch, LinkMatchError> {
        text.parse()
    }
}

impl std::fmt::Display for LinkMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkMatch::Url(url) => std::fmt::Display::fmt(url, f),
            LinkMatch::Fragment(fragment) => std::fmt::Display::fmt(fragment, f),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tag(String);

impl std::str::FromStr for Tag {
    type Err = TagError;

    fn from_str(text: &str) -> Result<Tag, TagError> {
        let length = text.chars().count();
        if (MIN_TAG..=MAX_TAG).contains(&length) {
            Ok(Tag(text.to_string()))
        } else {
            Err(TagError {
                rejected: text.to_string(),
            })
        }
    }
}

impl std::convert::TryFrom<&str> for Tag {
    type Error = TagError;

    fn try_from(text: &str) -> Result<Tag, TagError> {
        text.parse()
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for Tag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Tag {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Tag, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorkflowName(String);

impl std::str::FromStr for WorkflowName {
    type Err = WorkflowNameError;

    fn from_str(text: &str) -> Result<WorkflowName, WorkflowNameError> {
        let length = text.chars().count();
        if (MIN_WORKFLOW_NAME..=MAX_WORKFLOW_NAME).contains(&length) {
            Ok(WorkflowName(text.to_string()))
        } else {
            Err(WorkflowNameError {
                rejected: text.to_string(),
            })
        }
    }
}

impl std::convert::TryFrom<&str> for WorkflowName {
    type Error = WorkflowNameError;

    fn try_from(text: &str) -> Result<WorkflowName, WorkflowNameError> {
        text.parse()
    }
}

impl std::fmt::Display for WorkflowName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::Serialize for WorkflowName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for WorkflowName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<WorkflowName, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WorkflowStatus {
    Draft,
    Active,
    Paused,
    Draining,
    Stopped,
    Archived,
    Loading,
    Error,
}

const STATUSES: [WorkflowStatus; 8] = [
    WorkflowStatus::Draft,
    WorkflowStatus::Active,
    WorkflowStatus::Paused,
    WorkflowStatus::Draining,
    WorkflowStatus::Stopped,
    WorkflowStatus::Archived,
    WorkflowStatus::Loading,
    WorkflowStatus::Error,
];

impl WorkflowStatus {
    fn word(self) -> &'static str {
        match self {
            WorkflowStatus::Draft => "draft",
            WorkflowStatus::Active => "active",
            WorkflowStatus::Paused => "paused",
            WorkflowStatus::Draining => "draining",
            WorkflowStatus::Stopped => "stopped",
            WorkflowStatus::Archived => "archived",
            WorkflowStatus::Loading => "loading",
            WorkflowStatus::Error => "error",
        }
    }
}

impl std::str::FromStr for WorkflowStatus {
    type Err = UnknownStatus;

    fn from_str(text: &str) -> Result<WorkflowStatus, UnknownStatus> {
        STATUSES
            .into_iter()
            .find(|status| status.word() == text)
            .ok_or_else(|| UnknownStatus {
                rejected: text.to_string(),
            })
    }
}

impl std::convert::TryFrom<&str> for WorkflowStatus {
    type Error = UnknownStatus;

    fn try_from(text: &str) -> Result<WorkflowStatus, UnknownStatus> {
        text.parse()
    }
}

impl std::fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.word())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusChange {
    Active,
    Paused,
    Draining,
    Stopped,
}

impl StatusChange {
    fn word(self) -> &'static str {
        match self {
            StatusChange::Active => "active",
            StatusChange::Paused => "paused",
            StatusChange::Draining => "draining",
            StatusChange::Stopped => "stopped",
        }
    }
}

impl std::str::FromStr for StatusChange {
    type Err = StatusChangeError;

    fn from_str(text: &str) -> Result<StatusChange, StatusChangeError> {
        let status = text
            .parse::<WorkflowStatus>()
            .map_err(StatusChangeError::Unknown)?;
        StatusChange::try_from(status).map_err(StatusChangeError::Unsettable)
    }
}

impl std::convert::TryFrom<WorkflowStatus> for StatusChange {
    type Error = UnsettableStatus;

    fn try_from(status: WorkflowStatus) -> Result<StatusChange, UnsettableStatus> {
        match status {
            WorkflowStatus::Active => Ok(StatusChange::Active),
            WorkflowStatus::Paused => Ok(StatusChange::Paused),
            WorkflowStatus::Draining => Ok(StatusChange::Draining),
            WorkflowStatus::Stopped => Ok(StatusChange::Stopped),
            status => Err(UnsettableStatus { status }),
        }
    }
}

impl std::fmt::Display for StatusChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.word())
    }
}

impl serde::Serialize for StatusChange {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.word())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sharing {
    Enabled,
    Disabled,
}

impl std::str::FromStr for Sharing {
    type Err = SharingError;

    fn from_str(text: &str) -> Result<Sharing, SharingError> {
        match text {
            "yes" => Ok(Sharing::Enabled),
            "no" => Ok(Sharing::Disabled),
            _ => Err(SharingError {
                rejected: text.to_string(),
            }),
        }
    }
}

impl std::convert::TryFrom<&str> for Sharing {
    type Error = SharingError;

    fn try_from(text: &str) -> Result<Sharing, SharingError> {
        text.parse()
    }
}

impl std::fmt::Display for Sharing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Sharing::Enabled => "yes",
            Sharing::Disabled => "no",
        })
    }
}

impl serde::Serialize for Sharing {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(matches!(self, Sharing::Enabled))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SendCadence {
    Once,
    Recurring,
}

impl std::fmt::Display for SendCadence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SendCadence::Once => "false",
            SendCadence::Recurring => "true",
        })
    }
}

#[derive(serde::Serialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Rate(f64);

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct MessageTotals {
    pub total_sent: u64,
    pub total_opens: u64,
    pub unique_opens: u64,
    pub total_clicks: u64,
    pub unique_clicks: u64,
    pub total_bounces: u64,
}

impl MessageTotals {
    pub fn open_rate(&self) -> Option<Rate> {
        rate(self.unique_opens, self.total_sent)
    }

    pub fn click_rate(&self) -> Option<Rate> {
        rate(self.unique_clicks, self.total_sent)
    }
}

fn rate(measured: u64, sent: u64) -> Option<Rate> {
    if sent == 0 {
        return None;
    }
    Some(Rate(measured as f64 / sent as f64))
}

#[derive(serde::Serialize, Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ChangeCount(usize);

impl ChangeCount {
    pub const NONE: ChangeCount = ChangeCount(0);

    pub(super) fn over(edits: usize) -> ChangeCount {
        ChangeCount(edits)
    }
}

impl std::fmt::Display for ChangeCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

rejected_error!(
    DelayError,
    "a duration of one unit",
    "A value the step notation cannot express. One error per type, each naming the text it rejected."
);
rejected_error!(
    SendDaysError,
    "a set of send days",
    "Text that is not a day set."
);
rejected_error!(
    SendTimeError,
    "a time of day",
    "Text that is not a send time."
);
rejected_error!(
    TimezoneSourceError,
    "a timezone source",
    "Text that is not a timezone source."
);
rejected_error!(
    RecurrenceError,
    "a check interval",
    "Text that is not a check interval."
);
rejected_error!(TimezoneError, "a timezone", "Text that is not a timezone.");
rejected_error!(
    FeedUrlError,
    "an http or https feed url",
    "Text that is not a feed url."
);
rejected_error!(
    LinkUrlError,
    "an http or https url",
    "Text that is not a link url."
);
rejected_error!(
    LinkFragmentError,
    "a link fragment",
    "Text that is not a link fragment."
);
rejected_error!(
    LinkMatchError,
    "a link",
    "Text that is neither a url nor a fragment."
);
rejected_error!(TagError, "a tag", "Text that is not a tag.");
rejected_error!(
    WorkflowNameError,
    "a workflow name",
    "Text that is not a workflow name."
);
rejected_error!(
    UnknownStatus,
    "a workflow status",
    "Text that is not a workflow status."
);
rejected_error!(
    SharingError,
    "a sharing value",
    "Text that is not a sharing value."
);

#[derive(Clone, Debug)]
pub struct UnsettableStatus {
    status: WorkflowStatus,
}

impl std::fmt::Display for UnsettableStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}' is not a status a workflow can be set to",
            self.status
        )
    }
}

impl std::error::Error for UnsettableStatus {}

#[derive(Clone, Debug)]
pub enum StatusChangeError {
    Unknown(UnknownStatus),
    Unsettable(UnsettableStatus),
}

impl std::fmt::Display for StatusChangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusChangeError::Unknown(unknown) => std::fmt::Display::fmt(unknown, f),
            StatusChangeError::Unsettable(unsettable) => std::fmt::Display::fmt(unsettable, f),
        }
    }
}

impl std::error::Error for StatusChangeError {}
