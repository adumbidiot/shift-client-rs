use crate::util::extract_csrf_token;
use scraper::{Html, Selector};

/// Error that may occur while parsing a [`CodeRedemptionPage`]
#[derive(Debug, thiserror::Error)]
pub enum FromHtmlError {
    /// Missing csrf token
    #[error("missing csrf token")]
    MissingCsrfToken,

    /// Missing CheckRedemptionStatusUrl
    #[error("missing check redemption status")]
    MissingCheckRedemptionStatusUrl,
}

/// A code redemption page
#[derive(Debug)]
pub struct CodeRedemptionPage {
    /// The csrf token
    pub csrf_token: String,

    /// The check_redemption_status_url
    pub check_redemption_status_url: String,
}

impl CodeRedemptionPage {
    pub(crate) fn from_html(html: &Html) -> Result<Self, FromHtmlError> {
        let csrf_token = extract_csrf_token(html)
            .ok_or(FromHtmlError::MissingCsrfToken)?
            .to_string();

        let check_redemption_status_url_selector =
            Selector::parse("#check_redemption_status[data-url]")
                .expect("invalid check_redemption_status_url selector");
        let check_redemption_status_url = html
            .select(&check_redemption_status_url_selector)
            .next()
            .and_then(|element| element.value().attr("data-url"))
            .map(|url| format!("https://shift.gearboxsoftware.com{}", url))
            .ok_or(FromHtmlError::MissingCheckRedemptionStatusUrl)?;

        Ok(Self {
            csrf_token,
            check_redemption_status_url,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("../../../test_data/code_redemption_page_html.html");

    #[test]
    fn sample_1() {
        let html = Html::parse_document(SAMPLE_1);
        let _page = CodeRedemptionPage::from_html(&html).expect("invalid code redemption page");
    }
}
