mod issue_date;

use crate::code::Code;
use chrono::NaiveDate;
use issue_date::parse_issue_date;
use select::{
    node::Node,
    predicate::{
        Name,
        Text,
    },
};

pub const PC_CODE_INDEX: usize = 0;
pub const PLAYSTATION_CODE_INDEX: usize = 1;
pub const XBOX_CODE_INDEX: usize = 2;

// TODO: Needs overhaul to properly support bl3 instead of mashing bl3 into bl2-like stats
#[derive(Debug)]
pub struct ShiftCode {
    pub source: String,
    pub issue_date: NaiveDate,
    pub rewards: String,

    pub pc: Code,
    pub playstation: Code,
    pub xbox: Code,
}

impl ShiftCode {
    pub(crate) fn from_node(row: Node) -> Option<Self> {
        let mut iter = row.find(Name("td"));

        let source = iter
            .next()?
            .find(Text)
            .next()?
            .as_text()?
            .trim()
            .to_string();

        let rewards = process_rewards_node(iter.next()?)?;

        let issue_date_str = iter.next()?.find(Text).next()?.as_text()?.trim();
        let issue_date = parse_issue_date(issue_date_str).ok()?;

        let _expiration = iter.next()?;

        let pc = Code::from_node(iter.next()?)?;
        let playstation = Code::from_node(iter.next()?)?;
        let xbox = Code::from_node(iter.next()?)?;

        Some(ShiftCode {
            source,
            issue_date,
            rewards,

            pc,
            playstation,
            xbox,
        })
    }

    pub(crate) fn from_node_bl3(row: Node) -> Option<Self> {
        let mut iter = row.find(Name("td"));

        let source = iter
            .next()?
            .find(Text)
            .next()?
            .as_text()?
            .trim()
            .to_string();

        let rewards = process_rewards_node(iter.next()?)?;

        let issue_date_str = iter.next()?.find(Text).next()?.as_text()?.trim();
        let issue_date = parse_issue_date(issue_date_str).ok()?;

        let _expiration = iter.next()?;

        let code = Code::from_node(iter.next()?)?;

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

fn process_rewards_node(n: Node) -> Option<String> {
    let mut ret = n
        .find(Text)
        .filter_map(|text| text.as_text())
        .filter(|text| !text.trim().is_empty())
        .fold(String::new(), |mut ret, el| {
            ret += el;
            ret += " ";
            ret
        });

    while ret.chars().next_back().map_or(false, |c| c.is_whitespace()) {
        ret.pop();
    }

    Some(ret)
}
