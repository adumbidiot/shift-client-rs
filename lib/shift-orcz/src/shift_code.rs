mod issue_date;

use self::issue_date::parse_issue_date;
use crate::code::Code;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use time::Date;

pub const PC_CODE_INDEX: usize = 0;
pub const PLAYSTATION_CODE_INDEX: usize = 1;
pub const XBOX_CODE_INDEX: usize = 2;

static TD_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("td").expect("invalid TD_SELECTOR"));

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

    /// The issue date
    pub issue_date: Date,

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
        let issue_date = parse_issue_date(issue_date_str)?;

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
        let issue_date = parse_issue_date(&issue_date_str)?;

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
