/// Library result type
pub type ShiftResult<T> = Result<T, ShiftError>;

pub type RewardsPageError = crate::types::rewards::rewards_page::FromHtmlError;
pub type InvalidHomePageError = crate::types::home_page::FromHtmlError;
pub type RewardFormError = crate::types::rewards::reward_form::FromHtmlError;
pub type InvalidCodeRedemptionPageError =
    crate::types::rewards::code_redemption_page::FromHtmlError;
pub type InvalidAccountPage = crate::types::account_page::FromHtmlError;

/// The library error type
#[derive(Debug, thiserror::Error)]
pub enum ShiftError {
    /// Reqwest HTTP error
    #[error("reqwest http error")]
    Reqwest(#[from] reqwest::Error),

    /// Invalid HTTP Redirect
    #[error("invalid http redirect '{0}'")]
    InvalidRedirect(String),

    /// Json Error
    #[error("json parse error")]
    Json(#[from] serde_json::Error),

    /// Invalid Rewards page
    #[error("invalid rewards page")]
    InvalidRewardsPage(#[from] RewardsPageError),
    /// Invalid Home page
    #[error("invalid home page")]
    InvalidHomePage(#[from] InvalidHomePageError),
    /// Invalid RewardForm
    #[error("invalid reward form")]
    InvalidRewardForm(#[from] RewardFormError),
    /// Invalid code redemption page
    #[error("invalid code redemption page")]
    InvalidCodeRedemptionPage(#[from] InvalidCodeRedemptionPageError),
    /// Invalid Account page
    #[error("invalid account page")]
    InvalidAccountPage(#[from] InvalidAccountPage),

    /// Missing alert notice
    #[error("missing alert notice")]
    MissingAlertNotice,

    /// NonExistentShiftCode
    #[error("non-existent shift code")]
    NonExistentShiftCode,
    /// Expired ShiftCode
    #[error("expired shift code")]
    ExpiredShiftCode,
    /// Unavailable ShiftCode
    #[error("unavailable shift code")]
    UnavailableShiftCode,

    /// Shift Code already redeemed
    #[error("shift code already redeemed")]
    ShiftCodeAlreadyRedeemed,
    /// Launch shift game
    #[error("launch a shift game to redeem the code")]
    LaunchShiftGame,
    /// ShiftCode Redeem Fail
    #[error("failed to redeem shift code")]
    ShiftCodeRedeemFail,

    /// Failed to join tokio task
    #[error("tokio task join error")]
    TokioJoin(#[from] tokio::task::JoinError),
}
