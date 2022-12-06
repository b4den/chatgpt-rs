use std::time::Duration;

use anyhow::{anyhow, Result, Context};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE, USER_AGENT};
use serde_json::json;
use uuid::Uuid;

use crate::types::SessionResult;

#[derive(Debug, Clone)]
pub struct GPTClient {
    base_url: String,
    client: reqwest::Client,
    access_token: Option<String>,
    session_token: String,
}

impl GPTClient {
    pub fn new() -> Result<Self> {
        let token = std::env::var("GPT_SESSION")
            .expect("Please supply a valid token for GPT_SESSION env var");
        let headers = Self::construct_headers(&token);
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(100))
            .build()?;

        let base_url = "https://chat.openai.com/backend-api".to_string();

        Ok(GPTClient {
            base_url,
            client,
            session_token: token,
            access_token: None,
        })
    }

    pub async fn refresh_access_token(&mut self) -> Result<()> {
        let session_endpoint = "https://chat.openai.com/api/auth/session";
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(
                format!(
                    "__Secure-next-auth.session-token={}",
                    self.session_token.clone()
                )
                .as_str(),
            )
            .unwrap(),
        );

        let res = self
            .client
            .get(session_endpoint)
            .headers(headers)
            .send()
            .await?
            .json::<SessionResult>()
            .await?;

        self.access_token = Some(res.access_token);
        Ok(())
    }

    pub async fn post(&mut self, message: String) -> Result<String> {
        if self.access_token.is_none() {
            self.refresh_access_token().await?;
        }
        let access_token = self
            .access_token
            .as_ref()
            .expect("No access token provided");

        let uuidv4 = Uuid::new_v4();
        let conv_id = Uuid::new_v4();
        let params = json!(
            {
              "action": "next",
              "messages": [
                {
                  "id": uuidv4.to_string(),
                  "role": "user",
                  "content": {
                    "content_type": "text",
                    "parts": vec!(message),
                  }
                }
              ],
              "model": "text-davinci-002-render",
              "parent_message_id": conv_id.to_string(),

        });
        let res = self
            .client
            .post(format!("{}/{}", self.base_url, "conversation"))
            .json(&params)
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", access_token).as_str()).unwrap(),
            )
            .send()
            .await?;

        let response = res.text().await?;
        let start_field = "\"parts\": [\"";
        let end_field = "\"]}";

        let start = response.rfind(start_field);
        let end = response.rfind(end_field);

        let result = match (start, end) {
            (Some(s), Some(e)) if s < e => Some(response[s + start_field.len()..e].to_string()),
            _ => None,
        };

        result.context("Could not find a valid response")
    }

    fn construct_headers(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
            AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36",
            ),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers
    }
}
