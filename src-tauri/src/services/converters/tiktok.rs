use super::LinkConverterStrategy;
use regex::Regex;

pub struct TikTokConverter {
    vanilla_regex: Regex,
}

impl TikTokConverter {
    pub fn new() -> Self {
        Self {
            vanilla_regex: Regex::new(
                r"^(?:https?://)?(?:www\.)?(tiktok\.com)/@([^/]+)/video/([0-9]+)"
            ).unwrap(),
        }
    }
}

impl LinkConverterStrategy for TikTokConverter {
    fn platform_name(&self) -> &'static str {
        "tiktok"
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
        let video_id = caps.get(3)?.as_str();

        let converter_domain = match converter.to_lowercase().as_str() {
            "tfxktok" => "tfxktok.com",
            "tiktokez" => "tiktokez.com",
            _ => return None,
        };

        Some(format!(
            "https://{}/@{}/video/{}",
            converter_domain, username, video_id
        ))
    }

    fn available_converters(&self) -> Vec<&'static str> {
        vec!["tfxktok", "tiktokez"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiktok_matches() {
        let converter = TikTokConverter::new();
        
        assert!(converter.matches("https://www.tiktok.com/@leonardoexito86/video/7564994915109833998"));
        assert!(converter.matches("https://tiktok.com/@user/video/123456"));
        assert!(converter.matches("http://www.tiktok.com/@user/video/123456"));
        
        assert!(!converter.matches("https://tfxktok.com/@user/video/123456"));
    }

    #[test]
    fn test_tiktok_convert() {
        let converter = TikTokConverter::new();

        let result = converter.convert(
            "https://www.tiktok.com/@leonardoexito86/video/7564994915109833998",
            "tfxktok"
        );
        assert_eq!(result, Some("https://tfxktok.com/@leonardoexito86/video/7564994915109833998".to_string()));

        assert!(converter.convert("https://tfxktok.com/@user/video/123456", "tiktokez").is_none());
    }

    #[test]
    fn test_different_converter_outputs() {
        let converter = TikTokConverter::new();
        let vanilla_url = "https://www.tiktok.com/@user/video/123456";

        let expected = vec![
            ("tfxktok", "https://tfxktok.com/@user/video/123456"),
            ("tiktokez", "https://tiktokez.com/@user/video/123456"),
        ];

        for (converter_type, expected_url) in expected {
            let result = converter.convert(vanilla_url, converter_type);
            assert_eq!(result, Some(expected_url.to_string()));
        }
    }
}
