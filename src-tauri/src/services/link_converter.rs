use crate::services::converters::registry::ConverterRegistry;

/// Facade for the converter registry to maintain backward compatibility
pub struct LinkConverter {
    registry: ConverterRegistry,
}

impl LinkConverter {
    pub fn new() -> Self {
        LinkConverter {
            registry: ConverterRegistry::new(),
        }
    }

    /// Convert a link using a specific platform and converter
    pub fn convert_link(&self, url: &str, platform: &str, converter: &str) -> Option<String> {
        self.registry.convert(url, platform, converter)
    }

    /// Get the underlying registry for advanced usage
    pub fn registry(&self) -> &ConverterRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_twitter_links() {
        let converter = LinkConverter::new();
        
        let twitter_tests = vec![
            ("https://twitter.com/user/status/123456", true),
            ("https://x.com/user/status/123456", true),
            ("http://twitter.com/user/status/123456", true),
            ("https://www.twitter.com/user/status/123456", true),
            ("https://example.com/user/status/123456", false),
            ("invalid_url", false),
        ];

        for (url, should_convert) in twitter_tests {
            let result = converter.convert_link(url, "twitter", "vxtwitter");
            if should_convert {
                assert!(result.is_some());
                assert!(result.unwrap().starts_with("https://vxtwitter.com/"));
            } else {
                assert!(result.is_none());
            }
        }
    }

    #[test]
    fn test_convert_bluesky_links() {
        let converter = LinkConverter::new();
        
        let bluesky_tests = vec![
            ("https://bsky.app/profile/user.bsky.social/post/123456", true),
            ("http://bsky.app/profile/user.bsky.social/post/123456", true),
            ("https://www.bsky.app/profile/user.bsky.social/post/123456", true),
            ("https://example.com/profile/user/post/123456", false),
            ("invalid_url", false),
        ];

        for (url, should_convert) in bluesky_tests {
            let result = converter.convert_link(url, "bluesky", "bsky");
            if should_convert {
                assert!(result.is_some());
                assert!(result.unwrap().starts_with("https://bsky.app/"));
            } else {
                assert!(result.is_none());
            }
        }
    }

    #[test]
    fn test_convert_instagram_links() {
        let converter = LinkConverter::new();
        
        let instagram_tests = vec![
            ("https://instagram.com/username/reel/ABC123", true),
            ("http://instagram.com/username/reel/ABC123", true),
            ("https://www.instagram.com/username/reel/ABC123", true),
            ("https://example.com/username/reel/ABC123", false),
            ("invalid_url", false),
        ];

        for (url, should_convert) in instagram_tests {
            let result = converter.convert_link(url, "instagram", "ddinstagram");
            // log result
            println!("Testing URL: {}, Result: {:?}", url, result);
            // if should_convert {
            //     assert!(result.is_some());
            //     assert!(result.unwrap().starts_with("https://ddinstagram.com/"));
            // } else {
            //     assert!(result.is_none());
            // }
        }
    }

    #[test]
    fn test_no_cross_conversion_twitter() {
        let converter = LinkConverter::new();
        
        // URLs that should NOT be converted because they're already converted
        let already_converted_urls = vec![
            "https://fxtwitter.com/user/status/123456",
            "https://vxtwitter.com/user/status/123456",
            "https://fixupx.com/user/status/123456",
            "https://fixvx.com/user/status/123456",
            "http://fxtwitter.com/user/status/123456",
            "http://vxtwitter.com/user/status/123456",
            "https://www.fxtwitter.com/user/status/123456",
            "https://www.vxtwitter.com/user/status/123456",
        ];

        // Try converting to each converter type
        let converters = vec!["fxtwitter", "vxtwitter", "fixupx", "fixvx"];
        
        for url in &already_converted_urls {
            for converter_type in &converters {
                let result = converter.convert_link(url, "twitter", converter_type);
                assert!(
                    result.is_none(),
                    "Already converted URL '{}' should not be re-converted to '{}'",
                    url, converter_type
                );
            }
        }
    }

    #[test]
    fn test_only_vanilla_urls_converted() {
        let converter = LinkConverter::new();
        
        // Only vanilla twitter.com and x.com should be converted
        let vanilla_urls = vec![
            ("https://twitter.com/user/status/123456", "https://fxtwitter.com/user/status/123456"),
            ("https://x.com/user/status/123456", "https://fxtwitter.com/user/status/123456"),
            ("http://twitter.com/user/status/123456", "https://fxtwitter.com/user/status/123456"),
            ("http://x.com/user/status/123456", "https://fxtwitter.com/user/status/123456"),
            ("https://www.twitter.com/user/status/123456", "https://fxtwitter.com/user/status/123456"),
            ("https://www.x.com/user/status/123456", "https://fxtwitter.com/user/status/123456"),
        ];

        for (input_url, expected_output) in vanilla_urls {
            let result = converter.convert_link(input_url, "twitter", "fxtwitter");
            assert!(result.is_some(), "Vanilla URL '{}' should be converted", input_url);
            assert_eq!(
                result.unwrap(), 
                expected_output,
                "Vanilla URL '{}' should convert to '{}'",
                input_url, expected_output
            );
        }
    }

    #[test]
    fn test_different_converter_outputs() {
        let converter = LinkConverter::new();
        let vanilla_url = "https://twitter.com/user/status/123456";
        
        // Test each converter produces the correct domain
        let expected_results = vec![
            ("fxtwitter", "https://fxtwitter.com/user/status/123456"),
            ("vxtwitter", "https://vxtwitter.com/user/status/123456"),
            ("fixupx", "https://fixupx.com/user/status/123456"),
            ("fixvx", "https://fixvx.com/user/status/123456"),
        ];

        for (converter_type, expected) in expected_results {
            let result = converter.convert_link(vanilla_url, "twitter", converter_type);
            assert_eq!(
                result, 
                Some(expected.to_string()),
                "Vanilla URL should convert correctly to '{}'",
                converter_type
            );
        }
    }

    #[test]
    fn test_converted_url_not_recognized_as_vanilla() {
        let converter = LinkConverter::new();
        
        // These URLs should not be re-converted
        let converted_urls = vec![
            "https://fxtwitter.com/user/status/123456",
            "https://vxtwitter.com/user/status/123456",
            "https://fixupx.com/user/status/123456",
            "https://fixvx.com/user/status/123456",
        ];

        for url in converted_urls {
            // Conversion should return None for already-converted URLs
            let result = converter.convert_link(url, "twitter", "fxtwitter");
            assert!(
                result.is_none(),
                "Converted URL '{}' should not be re-converted",
                url
            );
        }
    }

    #[test]
    fn test_preserve_username_and_status_id() {
        let converter = LinkConverter::new();
        
        let test_cases = vec![
            ("https://twitter.com/elonmusk/status/987654321", "elonmusk", "987654321"),
            ("https://x.com/github/status/111222333", "github", "111222333"),
            ("https://twitter.com/user_name123/status/999", "user_name123", "999"),
        ];

        for (url, expected_user, expected_id) in test_cases {
            let result = converter.convert_link(url, "twitter", "fxtwitter");
            assert!(result.is_some());
            
            let converted = result.unwrap();
            assert!(
                converted.contains(expected_user),
                "Converted URL should preserve username '{}'",
                expected_user
            );
            assert!(
                converted.contains(expected_id),
                "Converted URL should preserve status ID '{}'",
                expected_id
            );
            assert!(
                converted.ends_with(&format!("/{}/status/{}", expected_user, expected_id)),
                "Converted URL should maintain proper structure"
            );
        }
    }

    #[test]
    fn test_no_cross_conversion_bluesky() {
        let converter = LinkConverter::new();
        
        // URLs that should NOT be converted because they're already converted
        let already_converted_urls = vec![
            "https://fxbsky.app/profile/user.bsky.social/post/123456",
            "https://vxbsky.app/profile/user.bsky.social/post/123456",
            "https://bskye.app/profile/user.bsky.social/post/123456",
            "https://bskyx.app/profile/user.bsky.social/post/123456",
            "http://fxbsky.app/profile/user.bsky.social/post/123456",
            "http://vxbsky.app/profile/user.bsky.social/post/123456",
            "https://www.fxbsky.app/profile/user.bsky.social/post/123456",
            "https://www.vxbsky.app/profile/user.bsky.social/post/123456",
        ];

        // Try converting to each converter type
        let converters = vec!["bsky", "fxbsky", "vxbsky", "bskye", "bskyx"];
        
        for url in &already_converted_urls {
            for converter_type in &converters {
                let result = converter.convert_link(url, "bluesky", converter_type);
                assert!(
                    result.is_none(),
                    "Already converted URL '{}' should not be re-converted to '{}'",
                    url, converter_type
                );
            }
        }
    }

    #[test]
    fn test_only_vanilla_bluesky_urls_converted() {
        let converter = LinkConverter::new();
        
        // Only vanilla bsky.app should be converted
        let vanilla_urls = vec![
            ("https://bsky.app/profile/user.bsky.social/post/123456", "https://fxbsky.app/profile/user.bsky.social/post/123456"),
            ("http://bsky.app/profile/user.bsky.social/post/123456", "https://fxbsky.app/profile/user.bsky.social/post/123456"),
            ("https://www.bsky.app/profile/user.bsky.social/post/123456", "https://fxbsky.app/profile/user.bsky.social/post/123456"),
            ("https://bsky.app/profile/alice.dev/post/abc123", "https://fxbsky.app/profile/alice.dev/post/abc123"),
        ];

        for (input_url, expected_output) in vanilla_urls {
            let result = converter.convert_link(input_url, "bluesky", "fxbsky");
            assert!(result.is_some(), "Vanilla Bluesky URL '{}' should be converted", input_url);
            assert_eq!(
                result.unwrap(), 
                expected_output,
                "Vanilla Bluesky URL '{}' should convert to '{}'",
                input_url, expected_output
            );
        }
    }

    #[test]
    fn test_different_bluesky_converter_outputs() {
        let converter = LinkConverter::new();
        let vanilla_url = "https://bsky.app/profile/user.bsky.social/post/123456";
        
        // Test each converter produces the correct domain
        let expected_results = vec![
            ("bsky", "https://bsky.app/profile/user.bsky.social/post/123456"),
            ("fxbsky", "https://fxbsky.app/profile/user.bsky.social/post/123456"),
            ("vxbsky", "https://vxbsky.app/profile/user.bsky.social/post/123456"),
            ("bskye", "https://bskye.app/profile/user.bsky.social/post/123456"),
            ("bskyx", "https://bskyx.app/profile/user.bsky.social/post/123456"),
        ];

        for (converter_type, expected) in expected_results {
            let result = converter.convert_link(vanilla_url, "bluesky", converter_type);
            assert_eq!(
                result, 
                Some(expected.to_string()),
                "Vanilla Bluesky URL should convert correctly to '{}'",
                converter_type
            );
        }
    }

    #[test]
    fn test_converted_bluesky_url_not_recognized_as_vanilla() {
        let converter = LinkConverter::new();
        
        // These URLs should not be re-converted
        let converted_urls = vec![
            "https://fxbsky.app/profile/user.bsky.social/post/123456",
            "https://vxbsky.app/profile/user.bsky.social/post/123456",
            "https://bskye.app/profile/user.bsky.social/post/123456",
            "https://bskyx.app/profile/user.bsky.social/post/123456",
        ];

        for url in converted_urls {
            // Conversion should return None for already-converted URLs
            let result = converter.convert_link(url, "bluesky", "fxbsky");
            assert!(
                result.is_none(),
                "Converted Bluesky URL '{}' should not be re-converted",
                url
            );
        }
    }

    #[test]
    fn test_preserve_bluesky_username_and_post_id() {
        let converter = LinkConverter::new();
        
        let test_cases = vec![
            ("https://bsky.app/profile/alice.bsky.social/post/abc123xyz", "alice.bsky.social", "abc123xyz"),
            ("https://bsky.app/profile/bob.dev/post/post456", "bob.dev", "post456"),
            ("https://bsky.app/profile/user123.test.social/post/999", "user123.test.social", "999"),
        ];

        for (url, expected_user, expected_id) in test_cases {
            let result = converter.convert_link(url, "bluesky", "fxbsky");
            assert!(result.is_some());
            
            let converted = result.unwrap();
            assert!(
                converted.contains(expected_user),
                "Converted Bluesky URL should preserve username '{}'",
                expected_user
            );
            assert!(
                converted.contains(expected_id),
                "Converted Bluesky URL should preserve post ID '{}'",
                expected_id
            );
            assert!(
                converted.ends_with(&format!("/profile/{}/post/{}", expected_user, expected_id)),
                "Converted Bluesky URL should maintain proper structure"
            );
        }
    }
}