use crate::util::extract_csrf_token;
use select::document::Document;

#[derive(Debug)]
pub enum FromDocError {
    MissingCsrfToken,
}

#[derive(Debug)]
pub struct HomePage {
    pub csrf_token: String,
}

impl HomePage {
    pub(crate) fn from_doc(doc: &Document) -> Result<Self, FromDocError> {
        let csrf_token = extract_csrf_token(doc)
            .ok_or(FromDocError::MissingCsrfToken)?
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
        let doc = Document::from(SAMPLE_1);
        let _page = HomePage::from_doc(&doc).unwrap();
    }
}
