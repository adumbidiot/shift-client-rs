use crate::util::extract_name_value;
use select::{
    document::Document,
    node::Node,
    predicate::Name,
};

pub type FromDocError = FromNodeError;

#[derive(Debug)]
pub enum FromNodeError {
    MissingUtf8,
    MissingAuthToken,
    MissingCode,
    MissingCheck,
    MissingService,
    MissingTitle,
    MissingCommit,
}

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
    pub(crate) fn from_doc(doc: &Document) -> Result<Vec<Self>, FromDocError> {
        doc.find(Name("form")).map(RewardForm::from_node).collect()
    }

    pub(crate) fn from_node(el: Node) -> Result<Self, FromNodeError> {
        let utf8 = extract_name_value(el, "utf8")
            .ok_or(FromNodeError::MissingUtf8)?
            .to_string();

        let authenticity_token = extract_name_value(el, "authenticity_token")
            .ok_or(FromNodeError::MissingAuthToken)?
            .to_string();

        let archway_code_redemption_code = extract_name_value(el, "archway_code_redemption[code]")
            .ok_or(FromNodeError::MissingCode)?
            .to_string();

        let archway_code_redemption_check =
            extract_name_value(el, "archway_code_redemption[check]")
                .ok_or(FromNodeError::MissingCheck)?
                .to_string();

        let archway_code_redemption_service =
            extract_name_value(el, "archway_code_redemption[service]")
                .ok_or(FromNodeError::MissingService)?
                .to_string();

        let archway_code_redemption_title =
            extract_name_value(el, "archway_code_redemption[title]")
                .ok_or(FromNodeError::MissingTitle)?
                .to_string();

        let commit = extract_name_value(el, "commit")
            .ok_or(FromNodeError::MissingCommit)?
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
        let doc = Document::from(SAMPLE_1);
        let _form = RewardForm::from_doc(&doc).unwrap();
    }
}
