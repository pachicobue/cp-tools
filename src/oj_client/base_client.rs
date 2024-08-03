use crate::config;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use ::{
    color_eyre::eyre::{eyre, Result},
    cookie_store::CookieStore,
    ureq::Agent,
    url::Url,
};

pub(crate) struct BaseClient {
    agent: Agent,
}

impl BaseClient {
    pub(crate) fn new() -> Result<Self> {
        log::debug!("Load cookie.");
        let cookie_store = match File::open(Self::cookie_store_path()) {
            Ok(file) => match CookieStore::load_json(BufReader::new(file)) {
                Ok(cookie_store) => cookie_store,
                Err(_) => {
                    log::warn!(
                        "Failed to cookie store. Use default.\n{}",
                        Self::cookie_store_path().to_string_lossy()
                    );
                    CookieStore::default()
                }
            },
            Err(_) => CookieStore::default(),
        };
        let agent = ureq::builder().cookie_store(cookie_store).build();
        Ok(Self { agent })
    }

    pub(crate) fn get(&self, url: &Url) -> Result<String> {
        log::trace!("GET {}", url);
        match self.agent.get(&url.as_str()).call() {
            Ok(resp) => Ok(resp.into_string()?),
            Err(_) => Err(eyre!("Failed to access {}.", url)),
        }
    }

    pub(crate) fn post(&self, url: &Url, form: &[(&str, &str)]) -> Result<String> {
        log::trace!("POST {}", url);
        match self.agent.post(&url.as_str()).send_form(form) {
            Ok(resp) => Ok(resp.into_string()?),
            Err(_) => Err(eyre!("Failed to access {}.", url)),
        }
    }

    fn cookie_store_path() -> PathBuf {
        config::dirs::workspace_dir().join("cookies.json")
    }

    fn save_cookies(&self) -> Result<()> {
        if let Ok(file) = File::create(Self::cookie_store_path()) {
            self.agent
                .cookie_store()
                .save_json(&mut BufWriter::new(file))
                .unwrap();
            log::debug!(
                "Cookie saved.\n{}",
                Self::cookie_store_path().to_string_lossy()
            );
        }
        Ok(())
    }
}

impl Drop for BaseClient {
    fn drop(&mut self) {
        if let Err(err) = self.save_cookies() {
            log::error!("Failed to save cookie.\n{}", err);
        }
    }
}
