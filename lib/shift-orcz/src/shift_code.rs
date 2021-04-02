mod issue_date;

use crate::code::Code;
use chrono::NaiveDate;
use issue_date::parse_issue_date;
use scraper::{
    ElementRef,
    Selector,
};

pub const PC_CODE_INDEX: usize = 0;
pub const PLAYSTATION_CODE_INDEX: usize = 1;
pub const XBOX_CODE_INDEX: usize = 2;

/// A shift code table entry wrapper
///
/// TODO: Needs overhaul to properly support bl3 instead of mashing bl3 into bl2-like stats
#[derive(Debug)]
pub struct ShiftCode {
    /// The source
    pub source: String,

    /// The issue date
    pub issue_date: NaiveDate,

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
    pub(crate) fn from_element(row: ElementRef) -> Option<Self> {
        let td_selector = Selector::parse("td").expect("invalid td selector");
        let mut iter = row.select(&td_selector);

        let source = iter.next()?.text().next()?.trim().to_string();

        let rewards = process_rewards_node(iter.next()?);

        let issue_date_str = iter.next()?.text().next()?.trim();
        let issue_date = parse_issue_date(issue_date_str).ok()?;

        let _expiration = iter.next()?;

        let pc = Code::from_element(iter.next()?)?;
        let playstation = Code::from_element(iter.next()?)?;
        let xbox = Code::from_element(iter.next()?)?;

        Some(ShiftCode {
            source,
            issue_date,
            rewards,

            pc,
            playstation,
            xbox,
        })
    }

    /// Parse a [`ShiftCode`] from a bl3 element.
    pub(crate) fn from_element_bl3(row: ElementRef) -> Option<Self> {
        let td_selector = Selector::parse("td").expect("invalid td selector");
        let mut iter = row.select(&td_selector);

        let source = iter.next()?.text().next()?.trim().to_string();

        let rewards = process_rewards_node(iter.next()?);

        let issue_date_str = iter.next()?.text().next()?.trim();
        let issue_date = parse_issue_date(issue_date_str).ok()?;

        let _expiration = iter.next()?;

        let code = Code::from_element(iter.next()?)?;

        let pc = code.clone();
        let playstation = code.clone();
        let xbox = code;

        Some(ShiftCode {
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
