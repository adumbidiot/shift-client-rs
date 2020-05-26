use reqwest::StatusCode;

pub type ShiftResult<T> = Result<T, ShiftError>;

pub type RewardsPageError = crate::types::rewards::rewards_page::FromDocError;
pub type RewardFormError = crate::types::rewards::reward_form::FromDocError;

#[derive(Debug)]
pub enum ShiftError {
    Reqwest(reqwest::Error),
    InvalidStatus(StatusCode),
    InvalidRedirect(String),
    Json(serde_json::Error),

    InvalidRewardsPage(RewardsPageError),
    InvalidHomePage(crate::types::home_page::FromDocError),
    InvalidRewardForm(RewardFormError),
    InvalidCodeRedemptionPage(crate::types::rewards::code_redemption_page::FromDocError),
    InvalidAccountPage(crate::types::account_page::FromDocError),

    NonExistentShiftCode,
    ExpiredShiftCode,
    UnavailableShiftCode,
}

impl From<reqwest::Error> for ShiftError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for ShiftError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<crate::types::home_page::FromDocError> for ShiftError {
    fn from(e: crate::types::home_page::FromDocError) -> Self {
        ShiftError::InvalidHomePage(e)
    }
}

impl From<RewardsPageError> for ShiftError {
    fn from(e: RewardsPageError) -> Self {
        Self::InvalidRewardsPage(e)
    }
}

impl From<RewardFormError> for ShiftError {
    fn from(e: RewardFormError) -> Self {
        Self::InvalidRewardForm(e)
    }
}

impl From<crate::types::rewards::code_redemption_page::FromDocError> for ShiftError {
    fn from(e: crate::types::rewards::code_redemption_page::FromDocError) -> Self {
        Self::InvalidCodeRedemptionPage(e)
    }
}

impl From<crate::types::account_page::FromDocError> for ShiftError {
    fn from(e: crate::types::account_page::FromDocError) -> Self {
        Self::InvalidAccountPage(e)
    }
}
