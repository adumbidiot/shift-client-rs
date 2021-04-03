use crate::util::extract_csrf_token;
use scraper::Html;

/// Error that may occur while parsing a [`HomePage`].
#[derive(Debug, thiserror::Error)]
pub enum FromHtmlError {
    /// Missing csrf token
    #[error("missing csrf token")]
    MissingCsrfToken,
}

/// The home page
#[derive(Debug)]
pub struct HomePage {
    /// The csrf token
    pub csrf_token: String,
}

impl HomePage {
    /// Parse a [`HomePage`] from html
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

    const SAMPLE_1: &str = include_str!("../../test_data/home.html");

    #[test]
    fn sample_1() {
        let html = Html::parse_document(SAMPLE_1);
        let _page = HomePage::from_html(&html).expect("failed to parse home page");
    }
}
