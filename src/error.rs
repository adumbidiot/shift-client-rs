use reqwest::StatusCode;

/// Library result type
pub type ShiftResult<T> = Result<T, ShiftError>;

pub type RewardsPageError = crate::types::rewards::rewards_page::FromDocError;
pub type InvalidHomePageError = crate::types::home_page::FromDocError;
pub type RewardFormError = crate::types::rewards::reward_form::FromDocError;
pub type InvalidCodeRedemptionPageError = crate::types::rewards::code_redemption_page::FromDocError;
pub type InvalidAccountPage = crate::types::account_page::FromDocError;

/// The library error type
#[derive(Debug, thiserror::Error)]
pub enum ShiftError {
    /// Reqwest HTTP error
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// invalid http status
    #[error("invalid http status '{0}'")]
    InvalidStatus(StatusCode),
    /// Invalid HTTP Redirect
    #[error("invalid http redirect '{0}'")]
    InvalidRedirect(String),

    /// Json Error
    #[error("{0}")]
    Json(#[from] serde_json::Error),

    /// Invalid Rewards page
    #[error("{0}")]
    InvalidRewardsPage(#[from] RewardsPageError),
    /// Invalid Home page
    #[error("{0}")]
    InvalidHomePage(#[from] InvalidHomePageError),
    /// Invalid RewardForm
    #[error("{0}")]
    InvalidRewardForm(#[from] RewardFormError),
    /// Invalid code redemption page
    #[error("{0}")]
    InvalidCodeRedemptionPage(#[from] InvalidCodeRedemptionPageError),
    /// Invalid Account page
    #[error("{0}")]
    InvalidAccountPage(#[from] InvalidAccountPage),

    /// NonExistentShiftCode
    #[error("non-existent shift code")]
    NonExistentShiftCode,
    /// Expired ShiftCode
    #[error("expired shift code")]
    ExpiredShiftCode,
    /// Unavailable ShiftCode
    #[error("unavailable shift code")]
    UnavailableShiftCode,

    /// Failed to join tokio task
    #[error("{0}")]
    TokioJoin(#[from] tokio::task::JoinError),
}
