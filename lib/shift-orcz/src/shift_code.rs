use crate::code::Code;
use chrono::NaiveDate;
use logos::Logos;
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

        let rewards = iter
            .next()?
            .find(Text)
            .filter_map(|text| text.as_text())
            .filter(|text| !text.trim().is_empty())
            .fold(String::new(), |mut ret, el| {
                ret += el;
                ret += " ";
                ret
            });

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

        let rewards = iter
            .next()?
            .find(Text)
            .filter_map(|text| text.as_text())
            .filter(|text| !text.trim().is_empty())
            .fold(String::new(), |mut ret, el| {
                ret += el;
                ret += " ";
                ret
            });

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

#[derive(Logos, Debug, PartialEq)]
enum IssueDateToken {
    #[token("January", |_| 1)]
    #[token("Jan", |_| 1)]
    #[token("February", |_| 2)]
    #[token("Feb", |_| 2)]
    #[token("March", |_| 3)]
    #[token("Mar", |_| 3)]
    #[token("April", |_| 4)]
    #[token("Apr", |_| 4)]
    #[token("May", |_| 5)]
    #[token("June", |_| 6)]
    #[token("Jun", |_| 6)]
    #[token("July", |_| 7)]
    #[token("Jul", |_| 7)]
    #[token("August", |_| 8)]
    #[token("Aug", |_| 8)]
    #[token("September", |_| 9)]
    #[token("Sept", |_| 9)]
    #[token("Sep", |_| 9)]
    #[token("October", |_| 10)]
    #[token("Oct", |_| 10)]
    #[token("November", |_| 11)]
    #[token("Nov", |_| 11)]
    #[token("December", |_| 12)]
    #[token("Dec", |_| 12)]
    Month(u32),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(u32),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(",", logos::skip)]
    #[token("(", logos::skip)]
    #[token(")", logos::skip)]
    #[token("thru", logos::skip)]
    #[token("st", logos::skip)]
    #[token("nd", logos::skip)]
    #[token("rd", logos::skip)]
    #[token("th", logos::skip)]
    #[token("...", logos::skip)]
    #[token("verified", logos::skip)]
    Error,
}

#[derive(Debug)]
enum ParseIssueDateError {
    MissingYear,
    MissingMonth,
    MissingDay,

    InvalidToken(String),
}

fn parse_issue_date(s: &str) -> Result<NaiveDate, ParseIssueDateError> {
    let mut month = None;
    let mut day = None;
    let mut year = None;

    let mut lexer = IssueDateToken::lexer(s);

    while let Some(token) = lexer.next() {
        match token {
            IssueDateToken::Month(n) => {
                if month.is_none() {
                    month = Some(n)
                }
            }
            IssueDateToken::Number(n) => match (day.is_none(), year.is_none()) {
                (true, true) => day = Some(n),
                (false, true) => year = Some(n),
                (false, false) => {}
                _ => {}
            },
            IssueDateToken::Error => {
                return Err(ParseIssueDateError::InvalidToken(lexer.slice().into()));
            }
        }
    }

    Ok(NaiveDate::from_ymd(
        year.ok_or(ParseIssueDateError::MissingYear)? as i32,
        month.ok_or(ParseIssueDateError::MissingMonth)?,
        day.ok_or(ParseIssueDateError::MissingDay)?,
    ))
}
