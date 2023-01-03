use crate::util::extract_csrf_token;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};

static ALERT_NOTICE_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".alert.notice p").expect("invalid ALERT_NOTICE_SELECTOR"));

/// Error that may occur while parsing a [`RewardsPage`].
#[derive(Debug, thiserror::Error)]
pub enum FromHtmlError {
    /// MissingCsrfToken
    #[error("missing csrf token")]
    MissingCsrfToken,

    /// Invalid alert notice
    #[error("invalid alert notice")]
    InvalidAlertNotice(#[from] FromElementError),
}

/// The rewards page
#[derive(Debug)]
pub struct RewardsPage {
    /// The csrf token
    pub csrf_token: String,

    /// An alert notice
    pub alert_notice: Option<AlertNotice>,
}

impl RewardsPage {
    /// Parse a [`RewardsPage`] from html
    pub(crate) fn from_html(html: &Html) -> Result<Self, FromHtmlError> {
        let csrf_token = extract_csrf_token(html)
            .ok_or(FromHtmlError::MissingCsrfToken)?
            .to_string();

        let alert_notice = html
            .select(&ALERT_NOTICE_SELECTOR)
            .next()
            .map(AlertNotice::from_element)
            .transpose()?;

        Ok(Self {
            csrf_token,
            alert_notice,
        })
    }
}

/// An error that may occur while parsing an AlertNotice
#[derive(Debug, thiserror::Error)]
pub enum FromElementError {
    /// Missing text
    #[error("missing text")]
    MissingText,

    #[error("unknown text '{0}'")]
    UnknownText(String),
}

#[derive(Debug)]
pub enum AlertNotice {
    /// A shift code was already redeemed
    ShiftCodeAlreadyRedeemed,

    /// Launch a shift game to redeem codes
    LaunchShiftGame,

    /// Redeemed a shift code
    ShiftCodeRedeemed,

    /// Redeem failed
    ShiftCodeRedeemFail,
}

impl AlertNotice {
    /// Parse an alert notice from an element
    fn from_element(el: ElementRef) -> Result<Self, FromElementError> {
        let text = el.text().next().ok_or(FromElementError::MissingText)?;

        match text {
            "This SHiFT code has already been redeemed" => Ok(Self::ShiftCodeAlreadyRedeemed),
            "To continue to redeem SHiFT codes, please launch a SHiFT-enabled title first!" => {
                Ok(Self::LaunchShiftGame)
            }
            "Your code was successfully redeemed" => Ok(Self::ShiftCodeRedeemed),
            "Failed to redeem your SHiFT code" => Ok(Self::ShiftCodeRedeemFail),
            _ => Err(FromElementError::UnknownText(text.to_string())),
        }
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
