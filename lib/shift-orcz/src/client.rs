use crate::{
    Game,
    OrczError,
    OrczResult,
    ShiftCode,
};
use chrono::NaiveDate;
use scraper::{
    Html,
    Selector,
};

/// Client
#[derive(Default, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Make a new [`Client`].
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }

    /// Make a [`Html`] from the provided url str, processing it with f on a threadpool.
    ///
    /// # Errors
    /// Returns an error if the website could not be fetched
    async fn get_html<F, T>(&self, url: &str, f: F) -> OrczResult<T>
    where
        F: Fn(Html) -> OrczResult<T> + Send + 'static,
        T: Send + 'static,
    {
        let text = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(tokio::task::spawn_blocking(move || f(Html::parse_document(text.as_str()))).await??)
    }

    /// Get the shift codes for a given game
    pub async fn get_shift_codes(&self, game: Game) -> OrczResult<Vec<ShiftCode>> {
        self.get_html(game.page_url(), move |html| {
            extract_shift_codes(&html, game).ok_or(OrczError::TableParse)
        })
        .await
    }
}

fn extract_shift_codes(html: &Html, game: Game) -> Option<Vec<ShiftCode>> {
    let table_selector = Selector::parse("table").expect("invalid table selector");
    let table = html.select(&table_selector).next()?;

    let table_body_selector = Selector::parse("tbody").expect("invalid table body selector");
    let table_body = table.select(&table_body_selector).next()?;

    let row_selector = Selector::parse("tr").expect("invalid row selector");
    let mut ret = if game.is_bl3() {
        table_body
            .select(&row_selector)
            .skip(1) // Skip title
            .map(ShiftCode::from_element_bl3)
            .collect::<Option<Vec<_>>>()?
    } else {
        table_body
            .select(&row_selector)
            .skip(1) // Skip title
            .map(ShiftCode::from_element)
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
mod test {
    use super::*;

    const BL_DOC: &str = include_str!("../test_data/bl-keys.html");
    const BL2_DOC: &str = include_str!("../test_data/bl2-keys.html");
    const BLPS_DOC: &str = include_str!("../test_data/blps-keys.html");
    const BL3_DOC: &str = include_str!("../test_data/bl3-keys.html");

    #[test]
    fn parse_bl() {
        let html = Html::parse_document(BL_DOC);
        let codes = extract_shift_codes(&html, Game::Borderlands).expect("bl parse failed");
        assert!(!codes.is_empty());
        dbg!(codes);
    }

    #[test]
    fn parse_bl2() {
        let html = Html::parse_document(BL2_DOC);
        let codes = extract_shift_codes(&html, Game::Borderlands2).expect("bl2 parse failed");
        assert!(!codes.is_empty());
        dbg!(codes);
    }

    #[test]
    fn parse_blps() {
        let html = Html::parse_document(BLPS_DOC);
        let codes =
            extract_shift_codes(&html, Game::BorderlandsPreSequel).expect("blps parse failed");
        assert!(!codes.is_empty());
        dbg!(codes);
    }

    #[test]
    fn parse_bl3() {
        let html = Html::parse_document(BL3_DOC);
        let codes = extract_shift_codes(&html, Game::Borderlands3).expect("bl3 parse failed");
        assert!(!codes.is_empty());
        dbg!(&codes);
    }
}
