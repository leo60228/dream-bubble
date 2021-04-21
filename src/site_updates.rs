use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SiteUpdate {
    pub timestamp: DateTime<Utc>,
    pub path: String,
    pub hash: String,
    pub size: usize,
    pub download_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SiteUpdates {
    #[serde(rename = "data")]
    updates: Vec<SiteUpdate>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SiteFile {
    Html,
    MainJs,
    TwoJs,
    MainCss,
}

impl SiteFile {
    pub fn matches(self, path: &str) -> bool {
        match self {
            Self::Html => path == "/",
            Self::MainJs => path.contains("/main.") && path.ends_with(".js"),
            Self::TwoJs => path.contains("/2.") && path.ends_with(".js"),
            Self::MainCss => path.contains("/main.") && path.ends_with(".css"),
        }
    }
}

impl SiteUpdates {
    pub async fn fetch() -> Result<Self> {
        Ok(
            reqwest::get("https://api.sibr.dev/chronicler/v1/site/updates")
                .await?
                .json()
                .await?,
        )
    }

    pub fn file_at(&self, file: SiteFile, time: DateTime<Utc>) -> Result<Option<Url>> {
        let update = if let Some(update) = self
            .updates
            .iter()
            .filter(|x| file.matches(&x.path) && x.timestamp <= time)
            .last()
        {
            update
        } else {
            return Ok(None);
        };
        let url = Url::parse(&format!(
            "https://api.sibr.dev/chronicler/v1{}",
            update.download_url
        ))?;
        Ok(Some(url))
    }

    pub fn path_at(&self, path: &str, time: DateTime<Utc>) -> Result<Option<Url>> {
        let mut file = None;

        for possible_type in &[SiteFile::MainJs, SiteFile::TwoJs, SiteFile::MainCss] {
            if possible_type.matches(path) {
                file = Some(*possible_type);
                break;
            }
        }

        let update = if let Some(update) = self
            .updates
            .iter()
            .filter(|x| if let Some(file) = file { file.matches(&x.path) } else { path == x.path } && x.timestamp <= time)
            .last()
        {
            update
        } else {
            return Ok(None);
        };
        let url = Url::parse(&format!(
            "https://api.sibr.dev/chronicler/v1{}",
            update.download_url
        ))?;
        Ok(Some(url))
    }
}
