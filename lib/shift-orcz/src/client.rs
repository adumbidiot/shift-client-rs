use crate::{Game, OrczResult, ShiftCode};
use once_cell::sync::Lazy;
use scraper::{Html, Selector};
use time::Date;

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
        tokio::task::spawn_blocking(move || f(Html::parse_document(text.as_str()))).await?
    }

    /// Get the shift codes for a given game
    pub async fn get_shift_codes(&self, game: Game) -> OrczResult<Vec<ShiftCode>> {
        self.get_html(game.page_url(), move |html| {
            Ok(extract_shift_codes(&html, game)?)
        })
        .await
    }
}

/// Error that may occur while extracting shift codes from html
#[derive(Debug, thiserror::Error)]
pub enum ExtractShiftCodesError {
    /// Missing table
    #[error("missing table")]
    MissingTable,

    /// Missing table body
    #[error("missing table body")]
    MissingTableBody,

    /// Invalid lookup date
    #[error("invalid lookup date `{0}`")]
    InvalidLookupDate(String, #[source] time::error::Parse),

    /// Invalid shift code
    #[error("invalid shift code")]
    InvalidShiftCode(#[from] crate::shift_code::FromElementError),

    /// Unknown error occured
    #[error("unknown error")]
    Unknown,
}

/// Extract shift codes from html
fn extract_shift_codes(html: &Html, game: Game) -> Result<Vec<ShiftCode>, ExtractShiftCodesError> {
    static TABLE_BODY_ROW_SELECTOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse("table tbody tr").expect("invalid TABLE_BODY_ROW_SELECTOR"));
    const SAME_CODE_AS_FORMAT: &[time::format_description::FormatItem<'static>] =
        time::macros::format_description!("[ month padding:none ]/[ day ]/[ year ]");

    let mut ret = if game.is_bl3() {
        html.select(&TABLE_BODY_ROW_SELECTOR)
            .skip(1) // Skip title
            .map(ShiftCode::from_element_bl3)
            .collect::<Result<Vec<_>, _>>()?
    } else {
        html.select(&TABLE_BODY_ROW_SELECTOR)
            .skip(1) // Skip title
            .map(|el| ShiftCode::from_element(el, game.is_bl()))
            .collect::<Result<Vec<_>, _>>()?
    };

    // I hate this
    for i in 0..ret.len() {
        for code_index in 0..ret[i].get_code_array().len() {
            // Fix "Same as {date}" entries...
            {
                let code_str = ret[i].get_code_array()[code_index].as_str();
                if let Some(code_str) = code_str.strip_prefix("Same code as ") {
                    let lookup_date = Date::parse(code_str, SAME_CODE_AS_FORMAT).map_err(|e| {
                        ExtractShiftCodesError::InvalidLookupDate(code_str.to_string(), e)
                    })?;

                    let mut resolved_code = ret
                        .iter()
                        .find(|el| el.issue_date == Some(lookup_date))
                        .ok_or(ExtractShiftCodesError::Unknown)?
                        .get_code(code_index)
                        .ok_or(ExtractShiftCodesError::Unknown)?
                        .as_str()
                        .to_string();

                    std::mem::swap(
                        ret[i]
                            .get_code_mut(code_index)
                            .ok_or(ExtractShiftCodesError::Unknown)?
                            .as_mut_string(),
                        &mut resolved_code,
                    );
                }
            }

            // Fix "See Key Above" entries
            {
                let code_str = ret[i].get_code_array()[code_index].as_str();
                if code_str.starts_with("See Key Above") {
                    let mut resolved_code = ret[i - 1]
                        .get_code(code_index)
                        .ok_or(ExtractShiftCodesError::Unknown)?
                        .as_str()
                        .to_string();

                    std::mem::swap(
                        ret[i]
                            .get_code_mut(code_index)
                            .ok_or(ExtractShiftCodesError::Unknown)?
                            .as_mut_string(),
                        &mut resolved_code,
                    );
                }
            }
        }
    }

    Ok(ret)
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
