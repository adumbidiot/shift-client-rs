use logos::Logos;
use time::Date;

#[derive(Logos, Debug, PartialEq)]
enum IssueDateToken {
    #[token("January", |_| 1)]
    #[token("Jan", |_| 1)]
    #[token("Janurary", |_| 1)]
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
    Month(u8),

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
    #[token("\u{a0}", logos::skip)]
    #[token("n/a", logos::skip)]
    Error,
}

/// Error that can occur while parsing an issue date
#[derive(Debug, thiserror::Error)]
pub enum ParseIssueDateError {
    // Missing year component
    #[error("missing year")]
    MissingYear,

    /// Missing month component
    #[error("missing month")]
    MissingMonth,

    /// Missing day component
    #[error("missing day")]
    MissingDay,

    #[error("invalid token `{0}`")]
    InvalidToken(String),

    #[error(transparent)]
    TimeComponentRange(#[from] time::error::ComponentRange),

    #[error("invalid year '{0}'")]
    InvalidYear(u32, #[source] std::num::TryFromIntError),

    #[error("invalid day '{0}'")]
    InvalidDay(u32, #[source] std::num::TryFromIntError),
}

pub fn parse_issue_date(s: &str) -> Result<Date, ParseIssueDateError> {
    let mut month: Option<u8> = None;
    let mut day: Option<u8> = None;
    let mut year: Option<i32> = None;

    let mut lexer = IssueDateToken::lexer(s);

    while let Some(token) = lexer.next() {
        match token {
            IssueDateToken::Month(n) => {
                if month.is_none() {
                    month = Some(n)
                }
            }
            IssueDateToken::Number(n) => match (day.is_none(), year.is_none()) {
                (true, true) => {
                    day = Some(
                        n.try_into()
                            .map_err(|e| ParseIssueDateError::InvalidDay(n, e))?,
                    )
                }
                (false, true) => {
                    year = Some(
                        n.try_into()
                            .map_err(|e| ParseIssueDateError::InvalidYear(n, e))?,
                    )
                }
                (false, false) => {}
                _ => {}
            },
            IssueDateToken::Error => {
                return Err(ParseIssueDateError::InvalidToken(lexer.slice().into()));
            }
        }
    }

    Ok(Date::from_calendar_date(
        year.ok_or(ParseIssueDateError::MissingYear)?,
        month.ok_or(ParseIssueDateError::MissingMonth)?.try_into()?,
        day.ok_or(ParseIssueDateError::MissingDay)?,
    )?)
}
