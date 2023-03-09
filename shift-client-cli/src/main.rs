mod util;

use crate::util::input;
use crate::util::input_yn;
use anyhow::{ensure, Context};
use reqwest::StatusCode;
use shift_client::{types::RewardsPage, Client, RewardForm, ShiftError};
use shift_orcz::Game;
use std::time::Duration;

async fn manual_loop(client: &Client) {
    let rewards_page = match client.get_rewards_page().await {
        Ok(p) => p,
        Err(error) => {
            eprintln!("Failed to get rewards page, got error: {error:#?}");
            return;
        }
    };

    loop {
        print!("Enter a shift code, or type 'exit' to exit: ");
        let code = input();

        if code.to_lowercase() == "exit" {
            println!("Exiting...");
            break;
        }

        if let Err(error) = redeem_code(client, &rewards_page, code.trim()).await {
            eprintln!("{error:?}");
            eprintln!();
        }
    }
}

async fn auto_loop(client: &Client) {
    let orcz_client = shift_orcz::Client::new();

    let rewards_page = match client.get_rewards_page().await {
        Ok(p) => p,
        Err(error) => {
            eprintln!("Failed to get rewards page, got error: {error:#?}");
            return;
        }
    };

    let game = loop {
        println!("What game do you want to target? (bl, bl2, blps, bl3)");
        let choice = input().to_lowercase();
        println!();

        match choice.as_str() {
            "bl" => break Game::Borderlands,
            "bl2" => break Game::Borderlands2,
            "blps" => break Game::BorderlandsPreSequel,
            "bl3" => break Game::Borderlands3,
            data => {
                eprintln!("\"{data}\" is not a valid option");
                eprintln!();
            }
        }
    };
    println!("Targeting game: {game:?}");

    let codes = match orcz_client
        .get_shift_codes(game)
        .await
        .context("Failed to get shift codes")
    {
        Ok(codes) => codes,
        Err(error) => {
            eprintln!("{error:?}");
            eprintln!();
            return;
        }
    };

    for shift_code in codes {
        for code in shift_code
            .get_code_array()
            .iter()
            .filter(|code| code.is_valid())
            .take(1)
        // IDK how other platforms redeem, seems buggy so I'll focus on PC
        {
            println!("Code: {}", code.as_str());
            println!("Reward: {}", shift_code.rewards);
            println!(
                "Issue Date: {}",
                shift_code
                    .issue_date
                    .map(|date| format!("{date}"))
                    .unwrap_or_else(|| "Unknown".into())
            );
            println!("Source: {}", shift_code.source);
            println!();

            println!("Redeeming code...");
            loop {
                match redeem_code(client, &rewards_page, code.as_str().trim()).await {
                    Ok(()) => {
                        break;
                    }
                    Err(error) => {
                        if let Some(ShiftError::Reqwest(e)) = error.downcast_ref::<ShiftError>() {
                            if let Some(StatusCode::TOO_MANY_REQUESTS) = e.status() {
                                eprintln!("Encountered 429, backing off for 60 seconds...");
                                tokio::time::sleep(Duration::from_secs(60)).await;
                                continue;
                            }
                        }

                        eprintln!("{error:?}");
                        eprintln!();

                        break;
                    }
                }
            }
            println!();
        }
    }
}

async fn redeem_code(
    client: &Client,
    rewards_page: &RewardsPage,
    code: &str,
) -> anyhow::Result<()> {
    let forms = client
        .get_reward_forms(rewards_page, code.trim())
        .await
        .context("failed to get code")?;

    ensure!(!forms.is_empty(), "No forms retrieved for code");

    for form in forms {
        match redeem_form(client, &form).await {
            Ok(()) => {}
            Err(error) => {
                eprintln!("{error:?}");
                eprintln!();
            }
        }
    }

    Ok(())
}

async fn redeem_form(client: &Client, form: &RewardForm) -> anyhow::Result<()> {
    let redeem_response = client.redeem(form).await.context("Failed to redeem code")?;
    println!("Redeemed code!");

    if let Some(redeem_response) = redeem_response {
        if let Some(text) = redeem_response.text {
            println!("Response: {text}");
        } else {
            eprintln!("Unknown Redeem Response: {redeem_response:#?}");
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    tokio_rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    let client = login_client().await;

    println!("Would you like to use manual mode? (Y/N)");
    if input_yn() {
        println!("Using manual mode...");
        manual_loop(&client).await;
    } else {
        println!("Using auto mode...");
        auto_loop(&client).await;
    }

    Ok(())
}

/// Login the client
async fn login_client() -> Client {
    loop {
        // get credentials
        print!("Email: ");
        let email = input();
        print!("Password: ");
        let password = input();
        println!();

        // make client
        let client = Client::new(email.trim().into(), password.trim().into());

        // try to log in
        match client.login().await.context("Login failed") {
            Ok(page) => {
                println!("Logged in!");
                println!();

                println!("Email: {}", page.email);
                println!("Display Name: {}", page.display_name);
                println!("First Name: {}", page.first_name);
                println!();

                break client;
            }
            Err(error) => {
                eprintln!("{error:?}");
                eprintln!();
            }
        }
    }
}
