use crate::{
    error::{
        ShiftError,
        ShiftResult,
    },
    types::{
        rewards::{
            CodeRedemptionJson,
            CodeRedemptionPage,
            RewardForm,
            RewardsPage,
        },
        AccountPage,
        HomePage,
    },
};
use select::document::Document;
use std::sync::{
    Arc,
    RwLock,
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
                .unwrap(),

            client_data: Arc::new(RwLock::new(ClientData { email, password })),
        }
    }

    /// Get the home page. Does not need authentication.
    async fn get_home_page(&self) -> ShiftResult<HomePage> {
        let res = self.client.get(HOME_URL).send().await?;

        let status = res.status();
        if !status.is_success() {
            return Err(ShiftError::InvalidStatus(status));
        }

        let doc = Document::from(res.text().await?.as_str());
        let home_page = HomePage::from_doc(&doc)?;

        Ok(home_page)
    }

    /// Logs in and allows making other requests
    pub async fn login(&self) -> ShiftResult<AccountPage> {
        let home_page = self.get_home_page().await?;
        let lock = self.client_data.read().unwrap();
        let res = self
            .client
            .post(SESSIONS_URL)
            .form(&[
                ("utf8", "✓"),
                ("authenticity_token", &home_page.csrf_token),
                ("user[email]", &lock.email),
                ("user[password]", &lock.password),
                ("commit", "SIGN IN"),
            ])
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            return Err(ShiftError::InvalidStatus(status));
        }

        match res.url().as_str() {
            header @ "https://shift.gearboxsoftware.com/home?redirect_to=false" => {
                return Err(ShiftError::InvalidRedirect(header.into()));
            }
            "https://shift.gearboxsoftware.com/account" => {}
            url => {
                return Err(ShiftError::InvalidRedirect(url.into()));
            }
        }

        let body = res.text().await?;
        let doc = Document::from(body.as_str());
        let account_page = AccountPage::from_doc(&doc)?;
        Ok(account_page)
    }

    pub async fn get_rewards_page(&self) -> ShiftResult<RewardsPage> {
        let res = self.client.get(REWARDS_URL).send().await?;

        let status = res.status();
        if !status.is_success() {
            return Err(ShiftError::InvalidStatus(status));
        }

        let body = res.text().await?;
        let doc = Document::from(body.as_str());
        let page = RewardsPage::from_doc(&doc)?;

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
            .await?;

        let status = res.status();
        if !status.is_success() {
            return Err(ShiftError::InvalidStatus(status));
        }

        let body = res.text().await?;

        match body.as_str().trim() {
            "This SHiFT code has expired" => return Err(ShiftError::ExpiredShiftCode),
            "This SHiFT code does not exist" => return Err(ShiftError::NonExistentShiftCode),
            "This code is not available for your account" => {
                return Err(ShiftError::UnavailableShiftCode)
            }
            _ => {}
        }

        let doc = Document::from(body.as_str());
        let forms = RewardForm::from_doc(&doc)?;

        Ok(forms)
    }

    pub async fn redeem(&self, form: &RewardForm) -> ShiftResult<CodeRedemptionJson> {
        let res = self
            .client
            .post(CODE_REDEMPTIONS_URL)
            .form(&form)
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            return Err(ShiftError::InvalidStatus(status));
        }

        let url = res.url().as_str();
        if !url.starts_with(CODE_REDEMPTIONS_URL) {
            return Err(ShiftError::InvalidRedirect(url.into()));
        }

        let body = res.text().await?;
        let doc = Document::from(body.as_str());
        let page = CodeRedemptionPage::from_doc(&doc)?;

        let res = loop {
            let res = self
                .client
                .get(&page.check_redemption_status_url)
                .header("X-CSRF-Token", &page.csrf_token)
                .header("X-Requested-With", "XMLHttpRequest")
                .send()
                .await?;

            let status = res.status();
            if !status.is_success() {
                return Err(ShiftError::InvalidStatus(status));
            }

            let body = res.text().await?;
            let json: CodeRedemptionJson = serde_json::from_str(&body)?;

            tokio::time::delay_for(std::time::Duration::from_secs(2)).await;

            if !json.in_progress() {
                break json;
            }
        };

        Ok(res)
    }
}

struct ClientData {
    email: String,
    password: String,
}
