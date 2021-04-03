use crate::util::extract_csrf_token;
use scraper::Html;

/// Error that may occur while parsing a [`RewardsPage`].
#[derive(Debug, thiserror::Error)]
pub enum FromHtmlError {
    /// MissingCsrfToken
    #[error("missing csrf token")]
    MissingCsrfToken,
}

/// The rewards page
#[derive(Debug)]
pub struct RewardsPage {
    /// The csrf token
    pub csrf_token: String,
}

impl RewardsPage {
    /// Parse a [`RewardsPage`] from html
    pub(crate) fn from_html(html: &Html) -> Result<Self, FromHtmlError> {
        let csrf_token = extract_csrf_token(html)
            .ok_or(FromHtmlError::MissingCsrfToken)?
            .to_string();

        Ok(Self { csrf_token })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("../../../test_data/rewards_page_1.html");

    #[test]
    fn sample_1() {
        let html = Html::parse_document(SAMPLE_1);
        let _page = RewardsPage::from_html(&html).unwrap();
    }
}
