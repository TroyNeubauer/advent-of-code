use std::sync::Arc;

use anyhow::Result;

pub struct Client {
    inner: reqwest::blocking::Client,
}

const BASE_URL: &str = "https://adventofcode.com";

impl Client {
    pub fn new(session: &str) -> Result<Self> {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        let url = BASE_URL.parse()?;
        jar.add_cookie_str(&format!("session={session}"), &url);

        let inner = reqwest::blocking::Client::builder()
            .cookie_provider(jar)
            .build()?;
        Ok(Self { inner })
    }

    /// Downloads the html page associated with the given year and day
    pub fn download_page(&mut self, year: u32, day: u32) -> Result<String> {
        let url = format!("{BASE_URL}/{year}/day/{day}");
        Ok(self.inner.get(url).send()?.text()?)
    }
}
