use std::time::Duration;

use serde_json::Value;

use crate::model::RumbleStats;

/// Client for the private Rumble livestream API.
///
/// The exact shape of the payload has changed a couple of times upstream, so
/// the parsing is deliberately tolerant: numbers may arrive as JSON numbers or
/// as strings, and the counters may sit either directly under `followers` /
/// `subscribers` or nested inside a small object.
#[derive(Debug, Clone)]
pub struct RumbleClient {
    http: reqwest::Client,
}

impl Default for RumbleClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RumbleClient {
    /// Creates a client with sensible timeouts.
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .connect_timeout(Duration::from_secs(10))
            .user_agent(concat!("RumbleDash/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap_or_default();
        Self { http }
    }

    /// Fetches the raw JSON body from `api_url`.
    pub async fn fetch_raw(&self, api_url: &str) -> crate::Result<Value> {
        let url = api_url.trim();
        if url.is_empty() {
            return Err(crate::Error::Config(
                "API URL not set. Please set your Rumble API URL in the dashboard.".into(),
            ));
        }

        let response = self.http.get(url).send().await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(crate::Error::Other(format!(
                "Rumble API responded with {status}"
            )));
        }

        serde_json::from_str(&body).map_err(crate::Error::from)
    }

    /// Fetches and interprets the follower / subscriber counters.
    pub async fn fetch_stats(&self, api_url: &str) -> crate::Result<RumbleStats> {
        let value = self.fetch_raw(api_url).await?;
        Ok(RumbleStats {
            followers: extract_count(&value, "followers", "num_followers"),
            subscribers: extract_count(&value, "subscribers", "num_subscribers"),
        })
    }
}

/// Pulls a counter out of the API payload.
///
/// `block` is the outer key (`followers`) and `key` the inner one
/// (`num_followers`). A number stored directly under `block` is accepted too.
fn extract_count(payload: &Value, block: &str, key: &str) -> u64 {
    let Some(section) = payload.get(block) else {
        return 0;
    };

    match section {
        // {"followers": 12}
        Value::Number(_) | Value::String(_) => as_u64(section).unwrap_or(0),
        // {"followers": {"num_followers": 12}}
        Value::Object(map) => {
            if let Some(value) = map.get(key) {
                if let Some(n) = as_u64(value) {
                    return n;
                }
            }
            // Some payloads use a different inner name, e.g. {"count": 12}.
            for alternative in ["count", "total", "value"] {
                if let Some(value) = map.get(alternative) {
                    if let Some(n) = as_u64(value) {
                        return n;
                    }
                }
            }
            0
        }
        _ => 0,
    }
}

/// Converts a JSON value into a `u64`, accepting numbers and numeric strings.
fn as_u64(value: &Value) -> Option<u64> {
    match value {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => text
            .split_whitespace()
            .next()
            .and_then(|n| n.parse::<u64>().ok()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_the_documented_payload() {
        let payload = json!({
            "followers": { "num_followers": 1234 },
            "subscribers": { "num_subscribers": 56 }
        });

        assert_eq!(extract_count(&payload, "followers", "num_followers"), 1234);
        assert_eq!(
            extract_count(&payload, "subscribers", "num_subscribers"),
            56
        );
    }

    #[test]
    fn accepts_plain_numbers() {
        let payload = json!({ "followers": 12, "subscribers": 3 });

        assert_eq!(extract_count(&payload, "followers", "num_followers"), 12);
        assert_eq!(extract_count(&payload, "subscribers", "num_subscribers"), 3);
    }

    #[test]
    fn accepts_numeric_strings() {
        let payload = json!({
            "followers": { "num_followers": "1234" },
            "subscribers": { "num_subscribers": " 56 " }
        });

        assert_eq!(extract_count(&payload, "followers", "num_followers"), 1234);
        assert_eq!(
            extract_count(&payload, "subscribers", "num_subscribers"),
            56
        );
    }

    #[test]
    fn accepts_alternative_inner_keys() {
        let payload = json!({
            "followers": { "count": 7 },
            "subscribers": { "total": 9 }
        });

        assert_eq!(extract_count(&payload, "followers", "num_followers"), 7);
        assert_eq!(extract_count(&payload, "subscribers", "num_subscribers"), 9);
    }

    #[test]
    fn missing_sections_are_zero() {
        let payload = json!({ "other": 1 });

        assert_eq!(extract_count(&payload, "followers", "num_followers"), 0);
        assert_eq!(extract_count(&payload, "subscribers", "num_subscribers"), 0);
    }

    #[tokio::test]
    async fn rejects_an_empty_url() {
        let client = RumbleClient::new();
        let error = client.fetch_stats("   ").await.unwrap_err();

        assert!(matches!(error, crate::Error::Config(_)));
    }
}
