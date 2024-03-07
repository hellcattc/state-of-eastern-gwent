use std::{error::Error, time::Duration};

pub struct GwentAPIConfig {
    max_retries: u8,
    timeout: Duration,
}

pub struct GwentAPIConfigBuilder {
    max_retries: Option<u8>,
    timeout: Option<Duration>,
}

impl GwentAPIConfigBuilder {
    pub fn new() -> GwentAPIConfigBuilder {
        GwentAPIConfigBuilder {
            max_retries: None,
            timeout: None,
        }
    }

    pub fn set_timeout(mut self, time: Duration) -> Self {
        self.timeout = Some(time);
        self
    }

    pub fn set_max_retries(mut self, max_retries: u8) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn build(self) -> GwentAPIConfig {
        let timeout = self.timeout.unwrap_or(Duration::from_millis(500));
        let max_retries = self.max_retries.unwrap_or(5);
        GwentAPIConfig {
            max_retries,
            timeout,
        }
    }
}

pub struct GwentAPI {
    client: reqwest::Client,
    requests_config: GwentAPIConfig,
}

impl GwentAPI {
    pub fn new(requests_config: GwentAPIConfig) -> GwentAPI {
        let client = reqwest::Client::new();
        GwentAPI {
            requests_config,
            client,
        }
    }

    fn build_request_to_url(&self, url: &str, options: &str) -> reqwest::RequestBuilder {
        let formatted_url = format!("{}{}", url, options);
        self.client
            .get(formatted_url)
            .timeout(self.requests_config.timeout)
    }

    async fn send_request_with_retries(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut retries: u8 = 0;
        let wait_time = Duration::from_secs(1);
        loop {
            let response = req.try_clone().unwrap().send().await;
            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        return Ok(res);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to fetch url {} on try {}", err, retries)
                }
            }

            retries += 1;
            if retries > self.requests_config.max_retries {
                return Err("Exceeded max retries".into());
            }

            let backoff_wait_time = wait_time.checked_mul(retries.into()).unwrap();
            println!("Retrying in {} seconds", backoff_wait_time.as_secs());
            tokio::time::sleep(backoff_wait_time).await;
        }
    }

    async fn fetch_from_guides_endpoint(
        &self,
        options: &str,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let req =
            self.build_request_to_url("https://www.playgwent.com/en/decks/api/guides/", options);
        self.send_request_with_retries(req).await
    }

    pub async fn get_guides(
        &self,
        offset: u16,
        limit: u16,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let response = self
            .fetch_from_guides_endpoint(&format!("offset/{}/limit/{}", offset, limit))
            .await;
        if let Err(_err) = response {
            return Err("Couldn't fetch guides".into());
        };
        return Ok(response.unwrap());
    }

    pub async fn get_deck(&self, id: u32) -> Result<reqwest::Response, Box<dyn Error>> {
        let response = self.fetch_from_guides_endpoint(&format!("{}", id)).await;
        if let Err(_err) = response {
            return Err("Couldn't fetch guides".into());
        };
        return Ok(response.unwrap());
    }
}
