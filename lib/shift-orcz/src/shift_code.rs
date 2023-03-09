mod issue_date;

use self::issue_date::parse_issue_date;
use crate::code::Code;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::ElementRef;
use scraper::Selector;
use time::Date;

pub const PC_CODE_INDEX: usize = 0;
pub const PLAYSTATION_CODE_INDEX: usize = 1;
pub const XBOX_CODE_INDEX: usize = 2;

static TD_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("td").expect("invalid TD_SELECTOR"));
static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"((?P<year_1>\d{4}).(?P<month_1>\d{2}).(?P<day_1>\d{2}))|((?P<month_2>[[:alpha:]]*?) (?P<day_2>\d{1,2}), (?P<year_2>\d{4}))").unwrap()
});

/// Error that may occur while parsing a ShiftCode from an element
#[derive(Debug, thiserror::Error)]
pub enum FromElementError {
    /// Missing the source
    #[error("missing source")]
    MissingSource,

    /// Missing rewards
    #[error("missing rewards")]
    MissingRewards,

    /// Missing Issue Date
    #[error("missing issue date")]
    MissingIssueDate,

    /// Missing expiration
    #[error("missing expiration")]
    MissingExpiration,

    /// Missing PC Code
    #[error("missing pc code")]
    MissingPcCode,

    /// Missing Playstaion Code
    #[error("missing playstation code")]
    MissingPlaystationCode,

    /// Missing PC Code
    #[error("missing xbox code")]
    MissingXboxCode,

    /// Invalid Issue date
    #[error("invalid issue date")]
    InvalidIssueDate(#[from] self::issue_date::ParseIssueDateError),

    /// Issue date missing year
    #[error("issue date missing year")]
    IssueDateMissingYear,

    /// Issue date missing month
    #[error("issue date missing month")]
    IssueDateMissingMonth,

    /// Issue date missing day
    #[error("issue date missing day")]
    IssueDateMissingDay,

    /// Issue date invalid year
    #[error("issue date missing year")]
    IssueDateInvalidYear(#[source] std::num::ParseIntError),

    /// Issue date invalid month
    #[error("issue date missing month int")]
    IssueDateInvalidMonthInt(#[source] std::num::ParseIntError),

    /// Issue date invalid month
    #[error("issue date missing month")]
    IssueDateInvalidMonth(#[source] time::error::ComponentRange),

    /// Invalid month str
    #[error("invalid month \"{0}\"")]
    IssueDateInvalidMonthStr(Box<str>),

    /// Issue date invalid day
    #[error("issue date missing day")]
    IssueDateInvalidDay(#[source] std::num::ParseIntError),

    /// Invalid issue date
    #[error("invalid date")]
    IssueDateInvalidDate(#[source] time::error::ComponentRange),

    /// Invalid code
    #[error("invalid code")]
    InvalidCode(#[from] crate::code::FromElementError),
}

/// A shift code table entry wrapper
///
/// TODO: Needs overhaul to properly support bl3 instead of mashing bl3 into bl2-like stats
#[derive(Debug)]
pub struct ShiftCode {
    /// The source
    pub source: String,

    /// The issue date.
    ///
    /// If None, it is unknown.
    pub issue_date: Option<Date>,

    /// The rewards
    pub rewards: String,

    /// The pc code
    pub pc: Code,

    /// The ps code
    pub playstation: Code,

    /// The xbox code
    pub xbox: Code,
}

impl ShiftCode {
    /// Parse a [`ShiftCode`] from a non-bl3 element.
    pub(crate) fn from_element(row: ElementRef, is_bl: bool) -> Result<Self, FromElementError> {
        let mut iter = row.select(&TD_SELECTOR);

        let source = iter
            .next()
            .and_then(|el| el.text().next())
            .ok_or(FromElementError::MissingSource)?
            .trim()
            .to_string();

        let rewards = process_rewards_node(iter.next().ok_or(FromElementError::MissingRewards)?);

        let issue_date_str = iter
            .next()
            .and_then(|el| el.text().next())
            .ok_or(FromElementError::MissingIssueDate)?
            .trim();
        let issue_date = if issue_date_str == "Unknown" {
            None
        } else if is_bl {
            let captures = DATE_REGEX
                .captures(dbg!(issue_date_str))
                .ok_or(FromElementError::MissingIssueDate)?;
            let y = captures
                .name("year_1")
                .or_else(|| captures.name("year_2"))
                .ok_or(FromElementError::IssueDateMissingYear)?
                .as_str()
                .parse::<i32>()
                .map_err(FromElementError::IssueDateInvalidYear)?;
            let m = captures
                .name("month_1")
                .map(|month| {
                    let month = month
                        .as_str()
                        .parse::<u8>()
                        .map_err(FromElementError::IssueDateInvalidMonthInt)?;
                    let month: time::Month = month
                        .try_into()
                        .map_err(FromElementError::IssueDateInvalidMonth)?;
                    Ok(month)
                })
                .or_else(|| {
                    captures.name("month_2").map(|month| match month.as_str() {
                        "January" | "Jan" => Ok(time::Month::January),
                        "February" | "Feb" => Ok(time::Month::February),
                        "March" | "Mar" => Ok(time::Month::March),
                        "April" | "Apr" => Ok(time::Month::April),
                        "May" => Ok(time::Month::May),
                        "June" => Ok(time::Month::June),
                        "July" => Ok(time::Month::July),
                        "August" => Ok(time::Month::August),
                        "September" => Ok(time::Month::September),
                        "October" | "Oct" => Ok(time::Month::October),
                        "November" | "Nov" => Ok(time::Month::November),
                        "December" | "Dec" => Ok(time::Month::December),
                        month => Err(FromElementError::IssueDateInvalidMonthStr(month.into())),
                    })
                })
                .ok_or(FromElementError::IssueDateMissingMonth)??;
            let d = captures
                .name("day_1")
                .or_else(|| captures.name("day_2"))
                .ok_or(FromElementError::IssueDateMissingDay)?
                .as_str()
                .parse::<u8>()
                .map_err(FromElementError::IssueDateInvalidDay)?;

            let date = Date::from_calendar_date(y, m, d)
                .map_err(FromElementError::IssueDateInvalidDate)?;

            Some(date)
        } else {
            Some(parse_issue_date(issue_date_str)?)
        };
        let _expiration = iter.next().ok_or(FromElementError::MissingExpiration)?;

        let pc = Code::from_element(iter.next().ok_or(FromElementError::MissingPcCode)?)?;
        let playstation = Code::from_element(
            iter.next()
                .ok_or(FromElementError::MissingPlaystationCode)?,
        )?;
        let xbox = Code::from_element(iter.next().ok_or(FromElementError::MissingXboxCode)?)?;

        Ok(ShiftCode {
            source,
            issue_date,
            rewards,

            pc,
            playstation,
            xbox,
        })
    }

    /// Parse a [`ShiftCode`] from a bl3 element.
    pub(crate) fn from_element_bl3(row: ElementRef) -> Result<Self, FromElementError> {
        let mut iter = row.select(&TD_SELECTOR);

        let source = iter
            .next()
            .and_then(|el| el.text().next())
            .ok_or(FromElementError::MissingSource)?
            .trim()
            .to_string();

        let rewards = process_rewards_node(iter.next().ok_or(FromElementError::MissingRewards)?);

        let issue_date_str = iter
            .next()
            .and_then(|el| el.text().next())
            .ok_or(FromElementError::MissingIssueDate)?
            .trim()
            .replace("??", "1"); // TODO: Consider making day optional
        let issue_date = if issue_date_str == "Unknown" {
            None
        } else {
            Some(parse_issue_date(&issue_date_str)?)
        };

        let _expiration = iter.next().ok_or(FromElementError::MissingExpiration)?;

        // Kinda hacky, maybe introduce a new error type
        let code = Code::from_element(iter.next().ok_or(FromElementError::MissingPcCode)?)?;

        let pc = code.clone();
        let playstation = code.clone();
        let xbox = code;

        Ok(ShiftCode {
            source,
            issue_date,
            rewards,

            pc,
            playstation,
            xbox,
        })
    }

    pub fn get_code_array(&self) -> [&Code; 3] {
        [&self.pc, &self.playstation, &self.xbox]
    }

    pub fn get_code_array_mut(&mut self) -> [&mut Code; 3] {
        [&mut self.pc, &mut self.playstation, &mut self.xbox]
    }

    pub fn get_code(&self, index: usize) -> Option<&Code> {
        Some(self.get_code_array()[index])
    }

    pub fn get_code_mut(&mut self, index: usize) -> Option<&mut Code> {
        Some(self.get_code_array_mut()[index])
    }
}

fn process_rewards_node(element: ElementRef) -> String {
    let mut ret =
        element
            .text()
            .filter(|text| !text.trim().is_empty())
            .fold(String::new(), |mut ret, el| {
                ret += el;
                ret += " ";
                ret
            });

    while ret.chars().next_back().map_or(false, |c| c.is_whitespace()) {
        ret.pop();
    }

    ret
}
