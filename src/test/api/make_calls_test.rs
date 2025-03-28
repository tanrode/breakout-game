
#[cfg(test)]
mod tests {
    use crate::api::make_calls::{validate_credentials, get_leaderboard, update_leaderboard};
    use mockito::{mock, Matcher};
    use serde_json::json;

    #[tokio::test]
    async fn test_validate_credentials_success() {
        let client = reqwest::Client::new();
        let _m = mock("POST", "/user/login")
            .match_body(r#"{"gamer_id":"test_user","password":"password123"}"#)
            .with_status(200)
            .create();

        let result = validate_credentials(&client, &mockito::server_url(), "test_user", "password123").await.unwrap();
        assert_eq!(result, true);

        _m.assert();
    }

    #[tokio::test]
    async fn test_validate_credentials_unauthorized() {
        let client = reqwest::Client::new();
        let _m = mock("POST", "/user/login")
            .match_body(r#"{"gamer_id":"test_user","password":"wrong_password"}"#)
            .with_status(401)
            .create();

        let result = validate_credentials(&client, &mockito::server_url(), "test_user", "wrong_password").await.unwrap();
        assert_eq!(result, false);

        _m.assert();
    }

    #[tokio::test]
    async fn test_get_leaderboard_success() {
        let client = reqwest::Client::new();
        let _m = mock("GET", "/leaderboard")
            .with_status(200)
            .with_body(r#"[{"gamer_id": "player1", "high_score": 27, "time": "02:10"}]"#)
            .create();

        let result = get_leaderboard(&client, &mockito::server_url()).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].gamer_id, "player1");
        assert_eq!(result[0].high_score, 27);
        assert_eq!(result[0].time, "02:10");

        _m.assert();
    }

    #[tokio::test]
    async fn test_get_leaderboard_error() {
        let client = reqwest::Client::new();
        let _m = mock("GET", "/leaderboard")
            .with_status(500)
            .create();

        let result = get_leaderboard(&client, &mockito::server_url()).await;
        assert!(result.is_err());

        _m.assert();
    }

    #[tokio::test]
    async fn test_update_leaderboard_success() {
        let client = reqwest::Client::new();
        let _m = mock("PATCH", "/update_stats")
            .match_body(Matcher::Json(json!({
                "gamer_id": "player1",
                "high_score": 27,
                "time": "02:10"
            })))
            .with_status(200)
            .with_body(r#"{"gamer_id": "player1", "high_score": 27, "time": "02:10"}"#)
            .create();

        let result = update_leaderboard(&client, &mockito::server_url(), "player1", 27, "02:10").await.unwrap();
        assert_eq!(result.gamer_id, "player1");
        assert_eq!(result.high_score, 27);
        assert_eq!(result.time, "02:10");

        _m.assert();
    }

    #[tokio::test]
    async fn test_update_leaderboard_error() {
        let client = reqwest::Client::new();
        let _m = mock("PATCH", "/update_stats")
            .with_status(500)
            .create();

        let result = update_leaderboard(&client, &mockito::server_url(), "player1", 27, "02:10").await;
        assert!(result.is_err());

        _m.assert();
    }
}