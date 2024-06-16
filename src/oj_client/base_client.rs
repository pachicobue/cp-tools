use crate::config;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    sync::Arc,
};
use ::{
    anyhow::{anyhow, Result},
    reqwest::{Client, Url},
    reqwest_cookie_store::{CookieStore, CookieStoreMutex},
};

pub(crate) struct BaseClient {
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl BaseClient {
    pub(crate) fn new() -> Result<Self> {
        log::debug!(
            "Loading cookie from {}...",
            Self::cookie_store_path().to_string_lossy()
        );
        let cookie_store = match File::open(Self::cookie_store_path()) {
            Ok(file) => match CookieStore::load_json(BufReader::new(file)) {
                Ok(cookie_store) => cookie_store,
                Err(_) => {
                    log::warn!(
                        "Failed to load {}.",
                        Self::cookie_store_path().to_string_lossy()
                    );
                    CookieStore::default()
                }
            },
            Err(_) => CookieStore::default(),
        };
        let cookie_store = Arc::new(CookieStoreMutex::new(cookie_store));
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&cookie_store))
            .build()?;
        Ok(Self {
            client,
            cookie_store: Arc::clone(&cookie_store),
        })
    }

    pub(crate) async fn get(&self, url: &Url) -> Result<String> {
        let resp = self.client.get(url.clone()).send();
        Ok(resp.await?.error_for_status()?.text().await?)
    }

    pub(crate) async fn post(&self, url: &Url, form: &[(&str, &str)]) -> Result<String> {
        let resp = self.client.post(url.clone()).form(form).send();
        Ok(resp.await?.error_for_status()?.text().await?)
    }

    fn cookie_store_path() -> PathBuf {
        config::dirs::workspace_dir().join("cookies.json")
    }

    fn save_cookies(&self) -> Result<()> {
        if let Ok(file) = File::create(Self::cookie_store_path()) {
            self.cookie_store
                .lock()
                .map_err(|e| anyhow!("{}", e))?
                .save_json(&mut BufWriter::new(file))
                .map_err(|e| anyhow!("{}", e))?;
            log::debug!("Cookie saved to {:?}.", Self::cookie_store_path());
        }
        Ok(())
    }
}

impl Drop for BaseClient {
    fn drop(&mut self) {
        if let Err(err) = self.save_cookies() {
            log::warn!("Failed to save cookie: {}.", err);
        }
    }
}
