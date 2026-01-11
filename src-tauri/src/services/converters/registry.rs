use super::{LinkConverterStrategy, twitter::TwitterConverter, bluesky::BlueSkyConverter, tiktok::TikTokConverter, instagram::InstagramConverter};
use std::sync::Arc;

/// Registry that manages all available link converter strategies
pub struct ConverterRegistry {
    converters: Vec<Arc<dyn LinkConverterStrategy>>,
}

impl ConverterRegistry {
    /// Creates a new registry with default converters
    pub fn new() -> Self {
        Self {
            converters: vec![
                Arc::new(TwitterConverter::new()),
                Arc::new(BlueSkyConverter::new()),
                Arc::new(TikTokConverter::new()),
                Arc::new(InstagramConverter::new()),
            ],
        }
    }

    /// Register a new converter strategy
    pub fn register(&mut self, converter: Arc<dyn LinkConverterStrategy>) {
        self.converters.push(converter);
    }

    /// Convert a URL using a specific platform and converter
    pub fn convert(&self, url: &str, platform: &str, converter: &str) -> Option<String> {
        self.converters
            .iter()
            .find(|c| c.platform_name() == platform)
            .and_then(|c| c.convert(url, converter))
    }

    /// Try to convert a URL by checking all registered platforms
    pub fn try_convert_any(&self, url: &str, converter: &str) -> Option<(String, String)> {
        for platform_converter in &self.converters {
            if platform_converter.matches(url) {
                if let Some(converted) = platform_converter.convert(url, converter) {
                    return Some((platform_converter.platform_name().to_string(), converted));
                }
            }
        }
        None
    }

    /// Get a converter by platform name
    pub fn get_converter(&self, platform: &str) -> Option<Arc<dyn LinkConverterStrategy>> {
        self.converters
            .iter()
            .find(|c| c.platform_name() == platform)
            .cloned()
    }

    /// Get all registered platform names
    pub fn platforms(&self) -> Vec<&str> {
        self.converters
            .iter()
            .map(|c| c.platform_name())
            .collect()
    }

    /// Get available converters for a specific platform
    pub fn available_converters(&self, platform: &str) -> Vec<&'static str> {
        self.get_converter(platform)
            .map(|c| c.available_converters())
            .unwrap_or_default()
    }
}

impl Default for ConverterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_platforms() {
        let registry = ConverterRegistry::new();
        let platforms = registry.platforms();
        
        assert!(platforms.contains(&"twitter"));
        assert!(platforms.contains(&"bluesky"));
        assert!(platforms.contains(&"tiktok"));
        assert!(platforms.contains(&"instagram"));
    }

    #[test]
    fn test_registry_convert_twitter() {
        let registry = ConverterRegistry::new();
        
        let result = registry.convert(
            "https://twitter.com/user/status/123456",
            "twitter",
            "fxtwitter"
        );
        
        assert_eq!(result, Some("https://fxtwitter.com/user/status/123456".to_string()));
    }

    #[test]
    fn test_registry_convert_bluesky() {
        let registry = ConverterRegistry::new();
        
        let result = registry.convert(
            "https://bsky.app/profile/user.bsky.social/post/123456",
            "bluesky",
            "fxbsky"
        );
        
        assert_eq!(result, Some("https://fxbsky.app/profile/user.bsky.social/post/123456".to_string()));
    }

    #[test]
    fn test_registry_convert_tiktok() {
        let registry = ConverterRegistry::new();
        
        let result = registry.convert(
            "https://www.tiktok.com/@user/video/123456",
            "tiktok",
            "tfxktok"
        );
        
        assert_eq!(result, Some("https://tfxktok.com/@user/video/123456".to_string()));
    }

    #[test]
    fn test_registry_convert_instagram() {
        let registry = ConverterRegistry::new();
        
        let result = registry.convert(
            "https://www.instagram.com/user/reel/ABC123",
            "instagram",
            "ddinstagram"
        );
        
        assert_eq!(result, Some("https://ddinstagram.com/user/reel/ABC123".to_string()));
    }

    #[test]
    fn test_registry_try_convert_any() {
        let registry = ConverterRegistry::new();
        
        // Twitter URL
        let result = registry.try_convert_any(
            "https://twitter.com/user/status/123456",
            "fxtwitter"
        );
        assert!(result.is_some());
        let (platform, converted) = result.unwrap();
        assert_eq!(platform, "twitter");
        assert_eq!(converted, "https://fxtwitter.com/user/status/123456");
        
        // BlueSky URL
        let result = registry.try_convert_any(
            "https://bsky.app/profile/user.bsky.social/post/123456",
            "fxbsky"
        );
        assert!(result.is_some());
        let (platform, converted) = result.unwrap();
        assert_eq!(platform, "bluesky");
        assert_eq!(converted, "https://fxbsky.app/profile/user.bsky.social/post/123456");
    }

    #[test]
    fn test_registry_available_converters() {
        let registry = ConverterRegistry::new();
        
        let twitter_converters = registry.available_converters("twitter");
        assert_eq!(twitter_converters, vec!["fxtwitter", "vxtwitter", "fixupx", "fixvx"]);
        
        let bluesky_converters = registry.available_converters("bluesky");
        assert_eq!(bluesky_converters, vec!["bsky", "fxbsky", "vxbsky", "bskye", "bskyx"]);
    }

    #[test]
    fn test_registry_unknown_platform() {
        let registry = ConverterRegistry::new();
        
        let result = registry.convert(
            "https://example.com/post/123",
            "unknown",
            "converter"
        );
        
        assert!(result.is_none());
    }
}
