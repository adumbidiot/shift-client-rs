use crate::util::extract_by_name;
use scraper::{ElementRef, Html, Selector};

/// Error that may occur while parsing a [`RewardForm`].
pub type FromHtmlError = FromElementError;

/// Error that may occur while parsing a [`RewardForm`].
#[derive(Debug, thiserror::Error)]
pub enum FromElementError {
    /// ?
    #[error("missing utf8")]
    MissingUtf8,
    /// Missing auth token
    #[error("missing auth token")]
    MissingAuthToken,
    /// Missing code
    #[error("missing code")]
    MissingCode,
    /// Missing check
    #[error("missing check")]
    MissingCheck,
    /// Missing service
    #[error("missing service")]
    MissingService,
    /// Missing title
    #[error("missing title")]
    MissingTitle,
    /// Missing commit
    #[error("missing commit")]
    MissingCommit,
}

/// The reward form
#[derive(Debug, serde::Serialize)]
pub struct RewardForm {
    utf8: String,
    authenticity_token: String,

    #[serde(rename = "archway_code_redemption[code]")]
    archway_code_redemption_code: String,

    #[serde(rename = "archway_code_redemption[check]")]
    archway_code_redemption_check: String,

    #[serde(rename = "archway_code_redemption[service]")]
    archway_code_redemption_service: String,

    #[serde(rename = "archway_code_redemption[title]")]
    archway_code_redemption_title: String,

    commit: String,
}

impl RewardForm {
    /// Parse a [`RewardForm`] from html
    pub(crate) fn from_html(html: &Html) -> Result<Vec<Self>, FromHtmlError> {
        let form_selector = Selector::parse("form").expect("invalid form selector");
        html.select(&form_selector)
            .map(RewardForm::from_element)
            .collect()
    }

    /// Parse a [`RewardForm`] from a node.
    pub(crate) fn from_element(element: ElementRef) -> Result<Self, FromElementError> {
        let utf8 = extract_by_name(element, "utf8")
            .ok_or(FromElementError::MissingUtf8)?
            .to_string();

        let authenticity_token = extract_by_name(element, "authenticity_token")
            .ok_or(FromElementError::MissingAuthToken)?
            .to_string();

        let archway_code_redemption_code =
            extract_by_name(element, "archway_code_redemption[code]")
                .ok_or(FromElementError::MissingCode)?
                .to_string();

        let archway_code_redemption_check =
            extract_by_name(element, "archway_code_redemption[check]")
                .ok_or(FromElementError::MissingCheck)?
                .to_string();

        let archway_code_redemption_service =
            extract_by_name(element, "archway_code_redemption[service]")
                .ok_or(FromElementError::MissingService)?
                .to_string();

        let archway_code_redemption_title =
            extract_by_name(element, "archway_code_redemption[title]")
                .ok_or(FromElementError::MissingTitle)?
                .to_string();

        let commit = extract_by_name(element, "commit")
            .ok_or(FromElementError::MissingCommit)?
            .to_string();

        Ok(Self {
            utf8,
            authenticity_token,
            archway_code_redemption_code,
            archway_code_redemption_check,
            archway_code_redemption_service,
            archway_code_redemption_title,
            commit,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE_1: &str = include_str!("../../../test_data/reward_form_1.html");

    #[test]
    fn sampe_1() {
        let doc = Html::parse_document(SAMPLE_1);
        let _form = RewardForm::from_html(&doc).expect("Failed to parse reward form");
    }
}
