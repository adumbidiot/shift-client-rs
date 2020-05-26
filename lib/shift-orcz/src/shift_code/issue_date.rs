use chrono::NaiveDate;
use logos::Logos;

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
pub enum ParseIssueDateError {
    MissingYear,
    MissingMonth,
    MissingDay,

    InvalidToken(String),
}

pub fn parse_issue_date(s: &str) -> Result<NaiveDate, ParseIssueDateError> {
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
