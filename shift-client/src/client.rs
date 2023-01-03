use crate::{
    error::{ShiftError, ShiftResult},
    types::{
        rewards::{AlertNotice, CodeRedemptionJson, CodeRedemptionPage, RewardForm, RewardsPage},
        AccountPage, HomePage,
    },
};
use scraper::Html;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

const HOME_URL: &str = "https://shift.gearboxsoftware.com/home";
const SESSIONS_URL: &str = "https://shift.gearboxsoftware.com/sessions";
const CODE_REDEMPTIONS_URL: &str = "https://shift.gearboxsoftware.com/code_redemptions";
const REWARDS_URL: &str = "https://shift.gearboxsoftware.com/rewards";
const ENTITLEMENT_OFFER_CODES_URL: &str =
    "https://shift.gearboxsoftware.com/entitlement_offer_codes";

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    client_data: Arc<RwLock<ClientData>>,
}

impl Client {
    /// Make a new shift client, not logged in
    pub fn new(email: String, password: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .expect("failed to build reqwest client"),

            client_data: Arc::new(RwLock::new(ClientData { email, password })),
        }
    }

    /// Get the home page. Does not need authentication.
    async fn get_home_page(&self) -> ShiftResult<HomePage> {
        let res = self.client.get(HOME_URL).send().await?;
        let home_page = res_to_html_transform(res, |html| Ok(HomePage::from_html(&html)?)).await?;
        Ok(home_page)
    }

    /// Logs in and allows making other requests
    pub async fn login(&self) -> ShiftResult<AccountPage> {
        let home_page = self.get_home_page().await?;

        let req = {
            let lock = self.client_data.read().expect("client data poisoned");
            self.client.post(SESSIONS_URL).form(&[
                ("utf8", "✓"),
                ("authenticity_token", &home_page.csrf_token),
                ("user[email]", &lock.email),
                ("user[password]", &lock.password),
                ("commit", "SIGN IN"),
            ])
        };
        let res = req.send().await?;

        match res.url().as_str() {
            "https://shift.gearboxsoftware.com/home?redirect_to=false" => {
                return Err(ShiftError::IncorrectEmailOrPassword);
            }
            "https://shift.gearboxsoftware.com/account" => {}
            url => {
                return Err(ShiftError::InvalidRedirect(url.into()));
            }
        }

        let account_page =
            res_to_html_transform(res, |html| Ok(AccountPage::from_html(&html)?)).await?;
        Ok(account_page)
    }

    /// Get the [`RewardsPage`]
    pub async fn get_rewards_page(&self) -> ShiftResult<RewardsPage> {
        let res = self.client.get(REWARDS_URL).send().await?;
        let page = res_to_html_transform(res, |html| Ok(RewardsPage::from_html(&html)?)).await?;
        Ok(page)
    }

    pub async fn get_reward_forms(
        &self,
        rewards_page: &RewardsPage,
        code: &str,
    ) -> ShiftResult<Vec<RewardForm>> {
        let res = self
            .client
            .get(ENTITLEMENT_OFFER_CODES_URL)
            .query(&[("code", code)])
            .header("X-CSRF-Token", &rewards_page.csrf_token)
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await?
            .error_for_status()?;

        let body = res.text().await?;

        match body.as_str().trim() {
            "This SHiFT code has expired" => return Err(ShiftError::ExpiredShiftCode),
            "This SHiFT code does not exist" => return Err(ShiftError::NonExistentShiftCode),
            "This code is not available for your account" => {
                return Err(ShiftError::UnavailableShiftCode)
            }
            _ => {}
        }

        let forms = tokio::task::spawn_blocking(move || {
            let html = Html::parse_document(body.as_str());
            RewardForm::from_html(&html)
        })
        .await??;

        Ok(forms)
    }

    /// Redeem a code
    pub async fn redeem(&self, form: &RewardForm) -> ShiftResult<Option<CodeRedemptionJson>> {
        let res = self
            .client
            .post(CODE_REDEMPTIONS_URL)
            .form(&form)
            .send()
            .await?;

        let url = res.url().as_str();
        if url.starts_with(REWARDS_URL) {
            let page =
                res_to_html_transform(res, |html| Ok(RewardsPage::from_html(&html)?)).await?;
            let alert_notice = page.alert_notice.ok_or(ShiftError::MissingAlertNotice)?;
            match alert_notice {
                AlertNotice::ShiftCodeAlreadyRedeemed => {
                    return Err(ShiftError::ShiftCodeAlreadyRedeemed);
                }
                AlertNotice::LaunchShiftGame => {
                    return Err(ShiftError::LaunchShiftGame);
                }
                AlertNotice::ShiftCodeRedeemed => {
                    return Ok(None);
                }
                AlertNotice::ShiftCodeRedeemFail => {
                    return Err(ShiftError::ShiftCodeRedeemFail);
                }
            }
        }

        if !url.starts_with(CODE_REDEMPTIONS_URL) {
            return Err(ShiftError::InvalidRedirect(url.into()));
        }

        let page =
            res_to_html_transform(res, |html| Ok(CodeRedemptionPage::from_html(&html)?)).await?;

        let res = loop {
            let json: CodeRedemptionJson = self
                .client
                .get(&page.check_redemption_status_url)
                .header("X-CSRF-Token", &page.csrf_token)
                .header("X-Requested-With", "XMLHttpRequest")
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            tokio::time::sleep(Duration::from_secs(2)).await;

            if !json.in_progress() {
                break json;
            }
        };

        Ok(Some(res))
    }
}

/// Client data
struct ClientData {
    email: String,
    password: String,
}

/// Convert a response to html, then feed it to the given transform function
async fn res_to_html_transform<F, T>(res: reqwest::Response, f: F) -> ShiftResult<T>
where
    F: Fn(Html) -> ShiftResult<T> + Send + 'static,
    T: Send + 'static,
{
    let text = res.error_for_status()?.text().await?;
    let ret = tokio::task::spawn_blocking(move || f(Html::parse_document(text.as_str()))).await??;
    Ok(ret)
}
