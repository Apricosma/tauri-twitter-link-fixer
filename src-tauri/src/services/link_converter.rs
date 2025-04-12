use crate::config::app_config::TwitterConverters;
use regex::Regex;

pub struct LinkConverter {
    twitter_regex: Regex,
}

impl LinkConverter {
    pub fn new() -> Self {
        LinkConverter {
            twitter_regex: Regex::new(r"(?:https?://)?(?:www\.)?(?:twitter\.com|x\.com)/([^/]+)/status/([0-9]+)").unwrap(),
        }
    }

    pub fn convert_twitter_link(&self, url: &str, converter: &TwitterConverters) -> Option<String> {
        if !self.twitter_regex.is_match(url) {
            return None;
        }

        let caps = self.twitter_regex.captures(url)?;
        let username = caps.get(1)?.as_str();
        let status_id = caps.get(2)?.as_str();

        let converter_domain = match converter {
            TwitterConverters::Fxtwitter => "fxtwitter.com",
            TwitterConverters::Vxtwitter => "vxtwitter.com",
            TwitterConverters::Fixupx => "fixupx.com",
            TwitterConverters::Fixvx => "fixvx.com",
        };

        Some(format!("https://{}/{}/status/{}", converter_domain, username, status_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_twitter_links() {
        let converter = LinkConverter::new();
        
        let test_cases = vec![
            ("https://twitter.com/user/status/123456", true),
            ("https://x.com/user/status/123456", true),
            ("http://twitter.com/user/status/123456", true),
            ("https://www.twitter.com/user/status/123456", true),
            ("https://example.com/user/status/123456", false),
            ("invalid_url", false),
        ];

        for (url, should_convert) in test_cases {
            let result = converter.convert_twitter_link(url, &TwitterConverters::Vxtwitter);
            if should_convert {
                assert!(result.is_some());
                assert!(result.unwrap().starts_with("https://vxtwitter.com/"));
            } else {
                assert!(result.is_none());
            }
        }
    }
}