use super::LinkConverterStrategy;
use regex::Regex;

pub struct InstagramConverter {
    vanilla_regex: Regex,
}

impl InstagramConverter {
    pub fn new() -> Self {
        Self {
      // Only matches vanilla instagram.com domain
      vanilla_regex: Regex::new(
        r"^https?:\/\/(?:www\.)?instagram\.com\/([^\/]+)\/reel\/([A-Za-z0-9_-]+)(?:\/|\?|$)"
      ).unwrap(),
    }
    }
}

impl LinkConverterStrategy for InstagramConverter {
    fn platform_name(&self) -> &'static str {
        "instagram"
    }

    fn matches(&self, url: &str) -> bool {
        self.vanilla_regex.is_match(url)
    }

    fn convert(&self, url: &str, converter: &str) -> Option<String> {
        if !self.matches(url) {
            return None;
        }

        let caps = self.vanilla_regex.captures(url)?;
        let username = caps.get(1)?.as_str();
        let post_id = caps.get(2)?.as_str();

        let converter_domain = match converter.to_lowercase().as_str() {
            "ddinstagram" => "ddinstagram.com",
            "kkinstagram" => "kkinstagram.com",
            "instagramez" => "instagramez.com",
            "eeinstagram" => "eeinstagram.com",
            _ => return None,
        };

        Some(format!(
            "https://{}/{}/reel/{}",
            converter_domain, username, post_id
        ))
    }

    fn available_converters(&self) -> Vec<&'static str> {
        vec!["ddinstagram", "kkinstagram", "instagramez", "eeinstagram"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instagram_matches() {
        let converter = InstagramConverter::new();

        assert!(converter.matches("https://www.instagram.com/username/reel/ABC123/"));
        assert!(!converter.matches("https://www.instagram.com/method_gg/p/DSaTYOqCIOg/"));

        // Test conversion not matching already converted links
        assert!(!converter.matches("https://ddinstagram.com/username/reel/ABC123/"));
        assert!(!converter.matches("https://kkinstagram.com/username/reel/ABC123/"));
        assert!(!converter.matches("https://instagramez.com/username/reel/ABC123/"));
        assert!(!converter.matches("https://eeinstagram.com/username/reel/ABC123/"));
    }

    #[test]
    fn test_instagram_conversion() {
        let converter = InstagramConverter::new();

        let result = converter.convert(
            "https://www.instagram.com/username/reel/ABC123/",
            "ddinstagram",
        );
        
        assert_eq!(
            result,
            Some("https://ddinstagram.com/username/reel/ABC123".to_string())
        );

        // Already converted links should return None
        assert!(converter
            .convert(
                "https://ddinstagram.com/username/reel/ABC123/",
                "kkinstagram"
            )
            .is_none());
    }
}
