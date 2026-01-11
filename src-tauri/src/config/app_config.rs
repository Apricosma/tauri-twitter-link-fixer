use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TikTokConverters {
    Tntok,
    Tfxktok,
    Tiktokez,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InstagramConverters {
    Ddinstagram,
    Kkinstagram,
    Instagramez,
    Eeinstagram,
}
// --- Platform Source Definitions ---

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "platform", content = "data")]
pub enum PlatformSource {
    #[serde(rename = "twitter")]
    Twitter(PlatformConverters<TwitterConverters>),
    #[serde(rename = "bluesky")]
    Bluesky(PlatformConverters<BlueskyConverters>),
    #[serde(rename = "tiktok")]
    Tiktok(PlatformConverters<TikTokConverters>),
    #[serde(rename = "instagram")]
    Instagram(PlatformConverters<InstagramConverters>),
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
                PlatformSource::Tiktok(PlatformConverters {
                    enabled: true,
                    converters: vec![
                        TikTokConverters::Tfxktok,
                        TikTokConverters::Tiktokez,
                    ],
                    selected: Some(TikTokConverters::Tfxktok),
                }),
                PlatformSource::Instagram(PlatformConverters {
                    enabled: true,
                    converters: vec![
                        InstagramConverters::Ddinstagram,
                        InstagramConverters::Kkinstagram,
                        InstagramConverters::Instagramez,
                        InstagramConverters::Eeinstagram,
                    ],
                    selected: Some(InstagramConverters::Kkinstagram),
                }),
            ],
        }
    }
}

impl SourcesConfig {
    pub fn from_file_or_default(path: &str) -> Result<Self, ConfigError> {
        let config_path = Path::new(path);

        // Development: Always recreate config
        let should_recreate = true;
        
        // Production: Only create if missing
        // let should_recreate = !config_path.exists();

        if should_recreate {
            println!("Recreating config file at {:?}", config_path);

            let default = Self::default();
            let yaml = serde_yaml::to_string(&default).expect("Failed to serialize default config");

            if let Some(parent) = config_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).expect("Failed to create config directory");
                }
            }

            fs::write(&config_path, yaml).expect("Failed to write default config file");
        }

        let settings = Config::builder()
            .add_source(File::with_name(path.strip_suffix(".yaml").unwrap_or(path)))
            .build()?;
        settings.try_deserialize()
    }

    pub fn save_to_file(&self, path: &str) {
        // Use a fixed path relative to the project root
        let config_path = Path::new(path);

        let yaml = serde_yaml::to_string(self).expect("Failed to serialize config");
        fs::write(&config_path, yaml).expect("Failed to write config file");
    }
}
