use super::LinkConverterStrategy;
use regex::Regex;

pub struct TwitterConverter {
    vanilla_regex: Regex,
}

impl TwitterConverter {
    pub fn new() -> Self {
        Self {
            // Only matches vanilla twitter.com and x.com domains
            vanilla_regex: Regex::new(
                r"^(?:https?://)?(?:www\.)?(twitter\.com|x\.com)/([^/]+)/status/([0-9]+)"
            ).unwrap(),
        }
    }
}

impl LinkConverterStrategy for TwitterConverter {
    fn platform_name(&self) -> &'static str {
        "twitter"
    }

    fn matches(&self, url: &str) -> bool {
        self.vanilla_regex.is_match(url)
    }

    fn convert(&self, url: &str, converter: &str) -> Option<String> {
        if !self.matches(url) {
            return None;
        }

        let caps = self.vanilla_regex.captures(url)?;
        let username = caps.get(2)?.as_str();
        let status_id = caps.get(3)?.as_str();

        let converter_domain = match converter.to_lowercase().as_str() {
            "fxtwitter" => "fxtwitter.com",
            "vxtwitter" => "vxtwitter.com",
            "fixupx" => "fixupx.com",
            "fixvx" => "fixvx.com",
            _ => return None,
        };

        Some(format!(
            "https://{}/{}/status/{}",
            converter_domain, username, status_id
        ))
    }

    fn available_converters(&self) -> Vec<&'static str> {
        vec!["fxtwitter", "vxtwitter", "fixupx", "fixvx"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_matches() {
        let converter = TwitterConverter::new();

        assert!(converter.matches("https://twitter.com/user/status/123456"));
        assert!(converter.matches("https://x.com/user/status/123456"));
        assert!(converter.matches("http://twitter.com/user/status/123456"));
        assert!(converter.matches("https://www.twitter.com/user/status/123456"));

        // Should not match already-converted URLs
        assert!(!converter.matches("https://fxtwitter.com/user/status/123456"));
        assert!(!converter.matches("https://vxtwitter.com/user/status/123456"));
        assert!(!converter.matches("https://example.com/user/status/123456"));
    }

    #[test]
    fn test_twitter_convert() {
        let converter = TwitterConverter::new();

        let result = converter.convert("https://twitter.com/user/status/123456", "fxtwitter");
        assert_eq!(result, Some("https://fxtwitter.com/user/status/123456".to_string()));

        let result = converter.convert("https://x.com/user/status/123456", "vxtwitter");
        assert_eq!(result, Some("https://vxtwitter.com/user/status/123456".to_string()));

        // Already converted should return None
        assert!(converter.convert("https://fxtwitter.com/user/status/123456", "vxtwitter").is_none());
    }

    #[test]
    fn test_different_converter_outputs() {
        let converter = TwitterConverter::new();
        let vanilla_url = "https://twitter.com/user/status/123456";

        let expected = vec![
            ("fxtwitter", "https://fxtwitter.com/user/status/123456"),
            ("vxtwitter", "https://vxtwitter.com/user/status/123456"),
            ("fixupx", "https://fixupx.com/user/status/123456"),
            ("fixvx", "https://fixvx.com/user/status/123456"),
        ];

        for (converter_type, expected_url) in expected {
            let result = converter.convert(vanilla_url, converter_type);
            assert_eq!(result, Some(expected_url.to_string()));
        }
    }

    #[test]
    fn test_preserve_username_and_id() {
        let converter = TwitterConverter::new();

        let test_cases = vec![
            ("https://twitter.com/elonmusk/status/987654321", "elonmusk", "987654321"),
            ("https://x.com/github/status/111222333", "github", "111222333"),
            ("https://twitter.com/user_name123/status/999", "user_name123", "999"),
        ];

        for (url, expected_user, expected_id) in test_cases {
            let result = converter.convert(url, "fxtwitter").unwrap();
            assert!(result.contains(expected_user));
            assert!(result.contains(expected_id));
            assert!(result.ends_with(&format!("/{}/status/{}", expected_user, expected_id)));
        }
    }
}
