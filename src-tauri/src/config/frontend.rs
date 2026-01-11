use serde::{Deserialize, Serialize};
use super::app_config::PlatformSource;

// --- Frontend Config Structures ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendPlatformMetadata {
    pub title: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendConverterConfig {
    pub enabled: bool,
    pub converters: Vec<String>,
    pub selected: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendPlatformSource {
    pub platform: String,
    pub metadata: FrontendPlatformMetadata,
    pub data: FrontendConverterConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendAppConfig {
    pub sources: Vec<FrontendPlatformSource>,
}

// --- Helper Functions ---

pub fn get_platform_metadata(platform: &str) -> FrontendPlatformMetadata {
    match platform {
        "twitter" => FrontendPlatformMetadata {
            title: "Twitter".to_string(),
            icon: "x".to_string(),
        },
        "bluesky" => FrontendPlatformMetadata {
            title: "BlueSky".to_string(),
            icon: "bluesky".to_string(),
        },
        "tiktok" => FrontendPlatformMetadata {
            title: "TikTok".to_string(),
            icon: "tiktok".to_string(),
        },
        "instagram" => FrontendPlatformMetadata {
            title: "Instagram".to_string(),
            icon: "instagram".to_string(),
        },
        "reddit" => FrontendPlatformMetadata {
            title: "Reddit".to_string(),
            icon: "reddit".to_string(),
        },
        _ => FrontendPlatformMetadata {
            title: platform.chars().next().unwrap().to_uppercase().to_string() + &platform[1..],
            icon: platform.to_string(),
        },
    }
}

pub fn transform_platform_source(source: &PlatformSource) -> FrontendPlatformSource {
    match source {
        PlatformSource::Twitter(data) => FrontendPlatformSource {
            platform: "twitter".to_string(),
            metadata: get_platform_metadata("twitter"),
            data: FrontendConverterConfig {
                enabled: data.enabled,
                converters: data.converters.iter().map(|c| format!("{:?}", c).to_lowercase()).collect(),
                selected: data.selected.as_ref().map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_default(),
            },
        },
        PlatformSource::Bluesky(data) => FrontendPlatformSource {
            platform: "bluesky".to_string(),
            metadata: get_platform_metadata("bluesky"),
            data: FrontendConverterConfig {
                enabled: data.enabled,
                converters: data.converters.iter().map(|c| format!("{:?}", c).to_lowercase()).collect(),
                selected: data.selected.as_ref().map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_default(),
            },
        },
        PlatformSource::Tiktok(data) => FrontendPlatformSource {
            platform: "tiktok".to_string(),
            metadata: get_platform_metadata("tiktok"),
            data: FrontendConverterConfig {
                enabled: data.enabled,
                converters: data.converters.iter().map(|c| format!("{:?}", c).to_lowercase()).collect(),
                selected: data.selected.as_ref().map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_default(),
            },
        },
        PlatformSource::Instagram(data) => FrontendPlatformSource {
            platform: "instagram".to_string(),
            metadata: get_platform_metadata("instagram"),
            data: FrontendConverterConfig {
                enabled: data.enabled,
                converters: data.converters.iter().map(|c| format!("{:?}", c).to_lowercase()).collect(),
                selected: data.selected.as_ref().map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_default(),
            },
        },
    }
}
