use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct CodeRedemptionJson {
    pub in_progress: Option<bool>,
    pub text: Option<String>,
    pub url: Option<String>,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

impl CodeRedemptionJson {
    pub fn in_progress(&self) -> bool {
        self.in_progress.unwrap_or(false)
    }

    pub fn is_success(&self) -> bool {
        match self.text.as_deref() {
            Some("Failed to redeem your SHiFT code") => false,
            Some("Your code was successfully redeemed") => true,
            Some(_s) => false,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("../../../test_data/code_redemption_json_1.json");
    const SAMPLE_2: &str = include_str!("../../../test_data/code_redemption_json_2.json");

    #[test]
    fn sample_1() {
        let _json: CodeRedemptionJson = serde_json::from_str(SAMPLE_1).unwrap();
    }

    #[test]
    fn sample_2() {
        let _json: CodeRedemptionJson = serde_json::from_str(SAMPLE_2).unwrap();
    }
}
