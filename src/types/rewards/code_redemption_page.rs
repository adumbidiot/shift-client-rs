use crate::util::extract_csrf_token;
use select::{
    document::Document,
    predicate::Attr,
};

#[derive(Debug)]
pub enum FromDocError {
    MissingCsrfToken,
    MissingCheckRedemptionStatusUrl,
}

#[derive(Debug)]
pub struct CodeRedemptionPage {
    pub csrf_token: String,
    pub check_redemption_status_url: String,
}

impl CodeRedemptionPage {
    pub(crate) fn from_doc(doc: &Document) -> Result<Self, FromDocError> {
        let csrf_token = extract_csrf_token(doc)
            .ok_or(FromDocError::MissingCsrfToken)?
            .to_string();

        let check_redemption_status_url = doc
            .find(Attr("id", "check_redemption_status"))
            .next()
            .and_then(|el| el.attr("data-url"))
            .map(|url| format!("https://shift.gearboxsoftware.com{}", url))
            .ok_or(FromDocError::MissingCheckRedemptionStatusUrl)?;

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
        let doc = Document::from(SAMPLE_1);
        let _page = CodeRedemptionPage::from_doc(&doc).unwrap();
    }
}
