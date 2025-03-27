use reqwest::{Client, StatusCode};
use serde_json::json;
use crate::client::models::structures::Leaderboard;

pub async fn validate_credentials( client: &Client, base_url: &str, gamer_id: &str, password: &str) -> Result<bool, reqwest::Error> {
    let body = json!({
        "gamer_id": gamer_id,
        "password": password,
    });

    let response = client.post(base_url.to_owned()+"/user/login")
        .json(&body)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::UNAUTHORIZED => Ok(false),
        _ => Err(response.error_for_status().unwrap_err()),
    }
}

pub async fn get_leaderboard(client: &Client, base_url: &str) -> Result<Vec<Leaderboard>, reqwest::Error> {
    let response = client.get(base_url.to_owned()+"/leaderboard")
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let api_response: Vec<Leaderboard> = response.json().await.unwrap();
            Ok(api_response)
        },
        _ => {
            Err(response.error_for_status().unwrap_err())
        },
    }
}

pub async fn update_leaderboard(client: &Client, base_url: &str, gamer_id: &str, high_score: i32, time_taken: &str) -> Result<Leaderboard, reqwest::Error> {
    let body = json!({
        "gamer_id": gamer_id,
        "high_score": high_score,
        "time": time_taken,
    });

    let response = client.patch(base_url.to_owned()+"/update_stats")
        .json(&body)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let api_response: Leaderboard = response.json().await.unwrap();
            Ok(api_response)
        },
        _ => {
            Err(response.error_for_status().unwrap_err())
        },
    }
}