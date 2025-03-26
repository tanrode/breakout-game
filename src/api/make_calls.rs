use reqwest::StatusCode;
use crate::client::models::structures::Leaderboard;

pub async fn validate_credentials(gamer_id: &str, password: &str) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_url = "http://127.0.0.1:8080/user/login"; // Replace with your API endpoint

    // Prepare the payload for the POST request
    let body = serde_json::json!({
        "gamer_id": gamer_id,
        "password": password,
    });

    // Send the request
    let response = client.post(api_url)
        .json(&body)
        .send()
        .await?;

    // Parse the response
    match response.status() {
        StatusCode::OK => {
                Ok(true) // Success case
        },
        StatusCode::UNAUTHORIZED => {
            Ok(false)
        },
        _ => {
            Err(response.error_for_status().unwrap_err())
        },
    }
}

pub async fn get_leaderboard() -> Result<Vec<Leaderboard>, reqwest::Error> {
    // todo!()
    let client = reqwest::Client::new();
    let api_url = "http://127.0.0.1:8080/leaderboard";

    let response = client.get(api_url)
        .send()
        .await?;

    // Parse the response
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

pub async fn update_leaderboard(gamer_id: &str, high_score: i32, time_taken: &str) -> Result<Leaderboard, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_url = "http://127.0.0.1:8080/update_stats";

    let body = serde_json::json!({
        "gamer_id": gamer_id,
        "high_score": high_score,
        "time": time_taken,
    });

    let response = client.patch(api_url)
        .json(&body)
        .send()
        .await?;

    // Parse the response
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