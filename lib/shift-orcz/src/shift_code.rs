use crate::code::Code;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::ElementRef;
use scraper::Selector;
use time::Date;

pub const PC_CODE_INDEX: usize = 0;
pub const PLAYSTATION_CODE_INDEX: usize = 1;
pub const XBOX_CODE_INDEX: usize = 2;

static TD_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("td").unwrap());
static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"((?P<year_1>\d{4}).(?P<month_1>\d{2}).(?P<day_1>\d{1,2}))|((?P<month_1_1>\d{1,2}).(?P<day_1_1>\d{2}).(?P<year_1_1>\d{4}))|((?P<month_2>[[:alpha:]]*?) *(?P<day_2>\d{1,2})(th|nd)? ?,? (?P<year_2>\d{4}))").unwrap()
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

    /// Failed to parse an issue date
    #[error("invalid issue date")]
    InvalidIssueDate {
        /// The date that could not be parsed
        date: Box<str>,

        /// The issue date error
        #[source]
        error: InvalidIssueDateError,
    },

    /// Invalid code
    #[error("invalid code")]
    InvalidCode(#[from] crate::code::FromElementError),
}

/// An error occured while parsing an issue date.
#[derive(Debug, thiserror::Error)]
pub enum InvalidIssueDateError {
    /// Failed to parse as it was an unknown format.
    #[error("unknown format")]
    UnknownFormat,

    /// Missing the year
    #[error("missing year")]
    MissingYear,

    /// Missing month
    #[error("missing month")]
    MissingMonth,

    /// Missing day
    #[error("missing day")]
    MissingDay,

    /// Invalid year
    #[error("invalid year")]
    InvalidYear(#[source] std::num::ParseIntError),

    /// Invalid month integer
    #[error("invalid month integer")]
    InvalidMonthInteger(#[source] std::num::ParseIntError),

    /// Invalid month
    #[error("invalid month")]
    InvalidMonth(#[source] time::error::ComponentRange),

    /// Invalid month string
    #[error("invalid month \"{0}\"")]
    InvalidMonthString(Box<str>),

    /// Invalid day
    #[error("invalid day")]
    InvalidDay(#[source] std::num::ParseIntError),

    /// The final parsed date is invalid
    #[error("invalid date")]
    InvalidDate(#[source] time::error::ComponentRange),
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
    pub(crate) fn from_element(row: ElementRef) -> Result<Self, FromElementError> {
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
        let issue_date = parse_issue_date_str(issue_date_str).map_err(|error| {
            FromElementError::InvalidIssueDate {
                date: issue_date_str.into(),
                error,
            }
        })?;
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
        let issue_date = parse_issue_date_str(&issue_date_str).map_err(|error| {
            FromElementError::InvalidIssueDate {
                date: issue_date_str.into(),
                error,
            }
        })?;

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

fn parse_issue_date_str(issue_date_str: &str) -> Result<Option<time::Date>, InvalidIssueDateError> {
    if issue_date_str == "Unknown" {
        return Ok(None);
    }

    let captures = DATE_REGEX
        .captures(issue_date_str)
        .ok_or(InvalidIssueDateError::UnknownFormat)?;
    let y = captures
        .name("year_1")
        .or_else(|| captures.name("year_1_1"))
        .or_else(|| captures.name("year_2"))
        .ok_or(InvalidIssueDateError::MissingYear)?
        .as_str()
        .parse::<i32>()
        .map_err(InvalidIssueDateError::InvalidYear)?;
    let m = captures
        .name("month_1")
        .or_else(|| captures.name("month_1_1"))
        .map(|month| {
            let month = month
                .as_str()
                .parse::<u8>()
                .map_err(InvalidIssueDateError::InvalidMonthInteger)?;
            let month: time::Month = month
                .try_into()
                .map_err(InvalidIssueDateError::InvalidMonth)?;
            Ok(month)
        })
        .or_else(|| {
            captures.name("month_2").map(|month| match month.as_str() {
                "January" | "Jan" => Ok(time::Month::January),
                "February" | "Feb" => Ok(time::Month::February),
                "March" | "Mar" => Ok(time::Month::March),
                "April" | "Apr" => Ok(time::Month::April),
                "May" => Ok(time::Month::May),
                "June" | "Jun" => Ok(time::Month::June),
                "July" | "Jul" => Ok(time::Month::July),
                "August" | "Aug" => Ok(time::Month::August),
                "September" | "Sep" | "Sept" => Ok(time::Month::September),
                "October" | "Oct" => Ok(time::Month::October),
                "November" | "Nov" => Ok(time::Month::November),
                "December" | "Dec" => Ok(time::Month::December),
                month => Err(InvalidIssueDateError::InvalidMonthString(month.into())),
            })
        })
        .ok_or(InvalidIssueDateError::MissingMonth)??;
    let d = captures
        .name("day_1")
        .or_else(|| captures.name("day_1_1"))
        .or_else(|| captures.name("day_2"))
        .ok_or(InvalidIssueDateError::MissingDay)?
        .as_str()
        .parse::<u8>()
        .map_err(InvalidIssueDateError::InvalidDay)?;

    let date = Date::from_calendar_date(y, m, d).map_err(InvalidIssueDateError::InvalidDate)?;

    Ok(Some(date))
}
