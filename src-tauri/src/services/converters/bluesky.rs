use super::LinkConverterStrategy;
use regex::Regex;

pub struct BlueSkyConverter {
    vanilla_regex: Regex,
}

impl BlueSkyConverter {
    pub fn new() -> Self {
        Self {
            // Only matches vanilla bsky.app domain
            vanilla_regex: Regex::new(
                r"^(?:https?://)?(?:www\.)?(bsky\.app)/profile/([^/]+)/post/([^/\s]+)"
            ).unwrap(),
        }
    }
}

impl LinkConverterStrategy for BlueSkyConverter {
    fn platform_name(&self) -> &'static str {
        "bluesky"
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
        let post_id = caps.get(3)?.as_str();

        let converter_domain = match converter.to_lowercase().as_str() {
            "bsky" => "bsky.app",
            "fxbsky" => "fxbsky.app",
            "vxbsky" => "vxbsky.app",
            "bskye" => "bskye.app",
            "bskyx" => "bskyx.app",
            _ => return None,
        };

        Some(format!(
            "https://{}/profile/{}/post/{}",
            converter_domain, username, post_id
        ))
    }

    fn available_converters(&self) -> Vec<&'static str> {
        vec!["bsky", "fxbsky", "vxbsky", "bskye", "bskyx"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluesky_matches() {
        let converter = BlueSkyConverter::new();

        assert!(converter.matches("https://bsky.app/profile/user.bsky.social/post/123456"));
        assert!(converter.matches("http://bsky.app/profile/user.bsky.social/post/123456"));
        assert!(converter.matches("https://www.bsky.app/profile/user.bsky.social/post/123456"));

        // Should not match already-converted URLs
        assert!(!converter.matches("https://fxbsky.app/profile/user.bsky.social/post/123456"));
        assert!(!converter.matches("https://vxbsky.app/profile/user.bsky.social/post/123456"));
        assert!(!converter.matches("https://example.com/profile/user/post/123456"));
    }

    #[test]
    fn test_bluesky_convert() {
        let converter = BlueSkyConverter::new();

        let result = converter.convert(
            "https://bsky.app/profile/user.bsky.social/post/123456",
            "fxbsky"
        );
        assert_eq!(
            result,
            Some("https://fxbsky.app/profile/user.bsky.social/post/123456".to_string())
        );

        // Already converted should return None
        assert!(converter.convert(
            "https://fxbsky.app/profile/user.bsky.social/post/123456",
            "vxbsky"
        ).is_none());
    }

    #[test]
    fn test_different_converter_outputs() {
        let converter = BlueSkyConverter::new();
        let vanilla_url = "https://bsky.app/profile/user.bsky.social/post/123456";

        let expected = vec![
            ("bsky", "https://bsky.app/profile/user.bsky.social/post/123456"),
            ("fxbsky", "https://fxbsky.app/profile/user.bsky.social/post/123456"),
            ("vxbsky", "https://vxbsky.app/profile/user.bsky.social/post/123456"),
            ("bskye", "https://bskye.app/profile/user.bsky.social/post/123456"),
            ("bskyx", "https://bskyx.app/profile/user.bsky.social/post/123456"),
        ];

        for (converter_type, expected_url) in expected {
            let result = converter.convert(vanilla_url, converter_type);
            assert_eq!(result, Some(expected_url.to_string()));
        }
    }

    #[test]
    fn test_preserve_username_and_post_id() {
        let converter = BlueSkyConverter::new();

        let test_cases = vec![
            ("https://bsky.app/profile/alice.bsky.social/post/abc123xyz", "alice.bsky.social", "abc123xyz"),
            ("https://bsky.app/profile/bob.dev/post/post456", "bob.dev", "post456"),
        ];

        for (url, expected_user, expected_id) in test_cases {
            let result = converter.convert(url, "fxbsky").unwrap();
            assert!(result.contains(expected_user));
            assert!(result.contains(expected_id));
            assert!(result.ends_with(&format!("/profile/{}/post/{}", expected_user, expected_id)));
        }
    }
}
