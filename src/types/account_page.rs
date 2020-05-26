use crate::util::extract_csrf_token;
use select::{
    document::Document,
    predicate::{
        Attr,
        Text,
    },
};

#[derive(Debug)]
pub enum FromDocError {
    MissingCsrfToken,
    MissingEmail,
    MissingDisplayName,
    MissingFirstName,
}

#[derive(Debug)]
pub struct AccountPage {
    pub csrf_token: String,
    pub email: String,
    pub display_name: String,
    pub first_name: String,
}

impl AccountPage {
    pub(crate) fn from_doc(doc: &Document) -> Result<Self, FromDocError> {
        let csrf_token = extract_csrf_token(doc)
            .ok_or(FromDocError::MissingCsrfToken)?
            .to_string();

        let email = get_text_by_id(doc, "current_email")
            .ok_or(FromDocError::MissingEmail)?
            .to_string();

        let display_name = get_text_by_id(doc, "current_display_name")
            .ok_or(FromDocError::MissingDisplayName)?
            .to_string();

        let first_name = get_text_by_id(doc, "current_first_name")
            .ok_or(FromDocError::MissingFirstName)?
            .to_string();

        Ok(Self {
            csrf_token,
            email,
            display_name,
            first_name,
        })
    }
}

fn get_text_by_id<'a>(doc: &'a Document, id: &str) -> Option<&'a str> {
    doc.find(Attr("id", id))
        .next()
        .and_then(|n| n.find(Text).next()?.as_text())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("../../test_data/account.html");

    #[test]
    fn sample_1() {
        let doc = Document::from(SAMPLE_1);
        let page = AccountPage::from_doc(&doc).unwrap();
        dbg!(page);
    }
}
