use select::{
    document::Document,
    predicate::{
        And,
        Attr,
        Name,
    },
};

#[derive(Debug)]
pub enum FromDocError {
    MissingCsrfToken,
}

#[derive(Debug)]
pub struct RewardsPage {
    pub csrf_token: String,
}

impl RewardsPage {
    pub(crate) fn from_doc(doc: &Document) -> Result<Self, FromDocError> {
        let csrf_token = doc
            .find(And(Name("meta"), Attr("name", "csrf-token")))
            .next()
            .ok_or(FromDocError::MissingCsrfToken)?
            .attr("content")
            .ok_or(FromDocError::MissingCsrfToken)?
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
        let doc = Document::from(SAMPLE_1);
        let _page = RewardsPage::from_doc(&doc).unwrap();
    }
}
