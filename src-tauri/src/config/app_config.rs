use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, File};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// --- Platform Types ---

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Twitter,
    Bluesky,
    Instagram,
    Reddit,
    Tiktok,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TwitterConverters {
    Fxtwitter,
    Vxtwitter,
    Fixupx,
    Fixvx,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BlueskyConverters {
    Bsky,
    Fxbsky,
    Vxbsky,
    Bskye,
    Bskyx,
}

// --- Platform Source Definitions ---

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "platform", content = "data")]
pub enum PlatformSource {
    #[serde(rename = "twitter")]
    Twitter(PlatformConverters<TwitterConverters>),
    #[serde(rename = "bluesky")]
    Bluesky(PlatformConverters<BlueskyConverters>),
    // Extend later for Instagram, Reddit, TikTok...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformConverters<T> {
    pub enabled: bool,
    pub converters: Vec<T>,
    pub selected: Option<T>,
}

// --- SourcesConfig ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourcesConfig {
    pub sources: Vec<PlatformSource>,
}

// --- Impl ---

impl Default for SourcesConfig {
    fn default() -> Self {
        SourcesConfig {
            sources: vec![
                PlatformSource::Twitter(PlatformConverters {
                    enabled: true,
                    converters: vec![
                        TwitterConverters::Fxtwitter,
                        TwitterConverters::Vxtwitter,
                        TwitterConverters::Fixupx,
                        TwitterConverters::Fixvx,
                    ],
                    selected: Some(TwitterConverters::Fxtwitter),
                }),
                PlatformSource::Bluesky(PlatformConverters {
                    enabled: true,
                    converters: vec![
                        BlueskyConverters::Bsky,
                        BlueskyConverters::Fxbsky,
                        BlueskyConverters::Vxbsky,
                        BlueskyConverters::Bskye,
                        BlueskyConverters::Bskyx,
                    ],
                    selected: Some(BlueskyConverters::Bsky),
                }),
            ],
        }
    }
}

impl SourcesConfig {
    pub fn from_file_or_default(path: &str) -> Result<Self, ConfigError> {
        if !Path::new(path).exists() {
            println!("Config file not found. Creating default config at {}", path);
            let default = Self::default();
            let yaml = serde_yaml::to_string(&default).expect("Failed to serialize default config");
            fs::write(path, yaml).expect("Failed to write default config file");
        }

        let settings = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        settings.try_deserialize()
    }

    pub fn save_to_file(&self, path: &str) {
        let yaml = serde_yaml::to_string(self).expect("Failed to serialize config");
        fs::write(path, yaml).expect("Failed to write config file");
    }

    pub fn set_selected_converter(&mut self, platform: Platform, converter_name: &str) -> Result<(), String> {
      for source in &mut self.sources {
          match (source, &platform) {
              (PlatformSource::Twitter(data), Platform::Twitter) => {
                  if let Some(found) = data.converters.iter().find(|c| format!("{:?}", c).to_lowercase() == converter_name.to_lowercase()) {
                      data.selected = Some(found.clone());
                      return Ok(());
                  } else {
                      return Err(format!("Converter '{}' not found in Twitter converters", converter_name));
                  }
              }
              (PlatformSource::Bluesky(data), Platform::Bluesky) => {
                  if let Some(found) = data.converters.iter().find(|c| format!("{:?}", c).to_lowercase() == converter_name.to_lowercase()) {
                      data.selected = Some(found.clone());
                      return Ok(());
                  } else {
                      return Err(format!("Converter '{}' not found in Bluesky converters", converter_name));
                  }
              }
              // Add similar match arms here later for Instagram, Reddit, TikTok...
              _ => {}
          }
      }
      Err(format!("Platform '{:?}' not found in sources", platform))
  }
}
