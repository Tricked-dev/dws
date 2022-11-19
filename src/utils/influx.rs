use futures_util::future::BoxFuture;

use crate::{config::CONFIG, error::Result};

async fn send_influx(event: &str, labels: &str, values: &str) -> Result<()> {
    if let Some(url) = &CONFIG.influx_url {
        let client = reqwest::Client::new();
        client
            .post(url)
            .body(format!("{}{} {}", event, labels, values))
            .send()
            .await?;
    }
    Ok(())
}

pub struct Influx {
    event: String,
    labels: String,
    values: String,
}

impl Influx {
    pub fn new(event: &str) -> Self {
        Self {
            event: event.to_owned(),
            labels: String::new(),
            values: "value=1".to_owned(),
        }
    }

    pub fn label(mut self, key: &str, value: &str) -> Self {
        if !self.labels.is_empty() {
            self.labels.push(',');
        }
        self.labels.push_str(key);
        self.labels.push('=');
        self.labels.push_str(value);
        self
    }

    pub fn value(mut self, key: &str, value: &str) -> Self {
        self.values.push(',');
        self.values.push_str(key);
        self.values.push('=');
        self.values.push_str(value);
        self
    }

    pub async fn send(self) -> Result<()> {
        send_influx(&self.event, &self.labels, &self.values).await
    }
}

impl std::future::IntoFuture for Influx {
    type Output = Result<()>;

    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
