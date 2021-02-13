pub mod code;
mod game;
pub mod shift_code;

pub use crate::{
    code::Code,
    game::Game,
    shift_code::ShiftCode,
};
use chrono::NaiveDate;
use select::{
    document::Document,
    predicate::Name,
};

/// Library Result Type
///
pub type OrczResult<T> = Result<T, OrczError>;

/// Library Error Type
///
#[derive(Debug, thiserror::Error)]
pub enum OrczError {
    /// Reqwest HTTP Error
    ///
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    
    /// Invalid HTTP StatusCode
    ///
    #[error("invalid status '{0}'")]
    InvalidStatus(reqwest::StatusCode),

    /// Error Parsing a Table
    /// This is usually a library error; update this lib.
    #[error("invalid table")]
    TableParse,
}

/// Client
///
#[derive(Default, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Make a new [`Client`].
    ///
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }

    /// Make a doc from the provided url str
    ///
    /// # Errors
    /// Returns an error if the website could not be fetched
    ///
    async fn get_doc(&self, url: &str) -> OrczResult<Document> {
        let res = self.client.get(url).send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(OrczError::InvalidStatus(status));
        }
        let text = res.text().await?;
        Ok(Document::from(text.as_str()))
    }

    /// Get the shift codes for a given game
    ///
    pub async fn get_shift_codes(&self, game: Game) -> OrczResult<Vec<ShiftCode>> {
        let doc = self.get_doc(game.page_url()).await?;
        extract_shift_codes(&doc, game).ok_or(OrczError::TableParse)
    }
}

fn extract_shift_codes(doc: &Document, game: Game) -> Option<Vec<ShiftCode>> {
    let table = doc.find(Name("table")).next()?;
    let table_body = table.find(Name("tbody")).next()?;

    let mut ret = if game.is_bl3() {
        table_body
            .find(Name("tr"))
            .skip(1) // Skip title
            .map(ShiftCode::from_node_bl3)
            .collect::<Option<Vec<_>>>()?
    } else {
        table_body
            .find(Name("tr"))
            .skip(1) // Skip title
            .map(ShiftCode::from_node)
            .collect::<Option<Vec<_>>>()?
    };

    // I hate this
    for i in 0..ret.len() {
        for code_index in 0..ret[i].get_code_array().len() {
            // Fix "Same as {date}" entries...
            {
                let code_str = ret[i].get_code_array()[code_index].as_str();
                if code_str.starts_with("Same code as ") {
                    let lookup_date =
                        NaiveDate::parse_from_str(code_str, "Same code as %m/%d/%Y").ok()?;

                    let mut resolved_code = ret
                        .iter()
                        .find(|el| el.issue_date == lookup_date)?
                        .get_code(code_index)?
                        .as_str()
                        .to_string();

                    std::mem::swap(
                        ret[i].get_code_mut(code_index)?.as_mut_string(),
                        &mut resolved_code,
                    );
                }
            }

            // Fix "See Key Above" entries
            {
                let code_str = ret[i].get_code_array()[code_index].as_str();
                if code_str.starts_with("See Key Above") {
                    let mut resolved_code = ret[i - 1].get_code(code_index)?.as_str().to_string();

                    std::mem::swap(
                        ret[i].get_code_mut(code_index)?.as_mut_string(),
                        &mut resolved_code,
                    );
                }
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BL_DOC: &str = include_str!("../test_data/bl-keys.html");
    const BL2_DOC: &str = include_str!("../test_data/bl2-keys.html");
    const BLPS_DOC: &str = include_str!("../test_data/blps-keys.html");
    const BL3_DOC: &str = include_str!("../test_data/bl3-keys.html");

    #[test]
    fn parse_bl() {
        let doc = Document::from(BL_DOC);
        let codes = extract_shift_codes(&doc, Game::Borderlands).unwrap();
        dbg!(codes);
    }

    #[test]
    fn parse_bl2() {
        let doc = Document::from(BL2_DOC);
        let codes = extract_shift_codes(&doc, Game::Borderlands2).unwrap();
        dbg!(codes);
    }

    #[test]
    fn parse_blps() {
        let doc = Document::from(BLPS_DOC);
        let codes = extract_shift_codes(&doc, Game::BorderlandsPreSequel).unwrap();
        dbg!(codes);
    }

    #[test]
    fn parse_bl3() {
        let doc = Document::from(BL3_DOC);
        let codes = extract_shift_codes(&doc, Game::Borderlands3).unwrap();
        dbg!(codes);
    }

    // Someone nuked the table online. Allow failure until fixed.
    /*
    #[tokio::test]
    async fn it_works_bl() {
        let client = Client::new();
        let codes = client.get_shift_codes(Game::Borderlands).await.unwrap();
        dbg!(codes);
    }
    */

    #[tokio::test]
    async fn it_works_bl2() {
        let client = Client::new();
        let codes = client.get_shift_codes(Game::Borderlands2).await.unwrap();
        dbg!(codes);
    }

    #[tokio::test]
    async fn it_works_blps() {
        let client = Client::new();
        let codes = client
            .get_shift_codes(Game::BorderlandsPreSequel)
            .await
            .unwrap();
        dbg!(codes);
    }

    #[tokio::test]
    async fn it_works_bl3() {
        let client = Client::new();
        let codes = client.get_shift_codes(Game::Borderlands3).await.unwrap();
        dbg!(codes);
    }
}
