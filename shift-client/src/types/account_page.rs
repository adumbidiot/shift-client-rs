use crate::util::extract_csrf_token;
use scraper::{Html, Selector};

/// Error that may occur while parsing an [`AccountPage`].
#[derive(Debug, thiserror::Error)]
pub enum FromHtmlError {
    /// Missing csrf token
    #[error("missing csrf token")]
    MissingCsrfToken,
    /// Missing email
    #[error("missing email")]
    MissingEmail,
    /// Missing display name
    #[error("missing display name")]
    MissingDisplayName,
    /// Missing first name
    #[error("missing first name")]
    MissingFirstName,
}

/// The account page
#[derive(Debug)]
pub struct AccountPage {
    /// The csrf token
    pub csrf_token: String,
    /// The email
    pub email: String,
    /// The display name
    pub display_name: String,
    /// The first name
    pub first_name: String,
}

impl AccountPage {
    /// Parse an [`AccountPage`] from html
    pub(crate) fn from_html(html: &Html) -> Result<Self, FromHtmlError> {
        let csrf_token = extract_csrf_token(html)
            .ok_or(FromHtmlError::MissingCsrfToken)?
            .to_string();

        let email = get_text_by_id(html, "current_email")
            .ok_or(FromHtmlError::MissingEmail)?
            .to_string();

        let display_name = get_text_by_id(html, "current_display_name")
            .ok_or(FromHtmlError::MissingDisplayName)?
            .to_string();

        let first_name = get_text_by_id(html, "current_first_name")
            .ok_or(FromHtmlError::MissingFirstName)?
            .to_string();

        Ok(Self {
            csrf_token,
            email,
            display_name,
            first_name,
        })
    }
}

fn get_text_by_id<'a>(html: &'a Html, id: &str) -> Option<&'a str> {
    let selector = Selector::parse(&format!("#{id}")).ok()?;
    let element = html.select(&selector).next()?;
    element.text().next()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("../../test_data/account.html");

    #[test]
    fn sample_1() {
        let html = Html::parse_document(SAMPLE_1);
        let page = AccountPage::from_html(&html).expect("invalid account page");
        dbg!(page);
    }
}
