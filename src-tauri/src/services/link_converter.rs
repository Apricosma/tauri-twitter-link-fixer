use regex::Regex;

pub struct LinkConverter {
    twitter_regex: Regex,
    bluesky_regex: Regex,
}

impl LinkConverter {
    pub fn new() -> Self {
        LinkConverter {
            twitter_regex: Regex::new(r"(?:https?://)?(?:www\.)?(?:twitter\.com|x\.com)/([^/]+)/status/([0-9]+)").unwrap(),
            bluesky_regex: Regex::new(r"(?:https?://)?(?:www\.)?bsky\.app/profile/([^/]+)/post/([^/\s]+)").unwrap(),
        }
    }

    pub fn convert_link(&self, url: &str, platform: &str, converter: &str) -> Option<String> {
        match platform {
            "twitter" => self.convert_twitter_link(url, converter),
            "bluesky" => self.convert_bluesky_link(url, converter),
            _ => None,
        }
    }

    fn convert_twitter_link(&self, url: &str, converter: &str) -> Option<String> {
        if !self.twitter_regex.is_match(url) {
            return None;
        }

        let caps = self.twitter_regex.captures(url)?;
        let username = caps.get(1)?.as_str();
        let status_id = caps.get(2)?.as_str();

        let converter_domain = match converter.to_lowercase().as_str() {
            "fxtwitter" => "fxtwitter.com",
            "vxtwitter" => "vxtwitter.com",
            "fixupx" => "fixupx.com",
            "fixvx" => "fixvx.com",
            _ => return None,
        };

        Some(format!("https://{}/{}/status/{}", converter_domain, username, status_id))
    }

    fn convert_bluesky_link(&self, url: &str, converter: &str) -> Option<String> {
        if !self.bluesky_regex.is_match(url) {
            return None;
        }

        let caps = self.bluesky_regex.captures(url)?;
        let username = caps.get(1)?.as_str();
        let post_id = caps.get(2)?.as_str();

        let converter_domain = match converter.to_lowercase().as_str() {
            "bsky" => "bsky.app",
            "fxbsky" => "fxbsky.app",
            "vxbsky" => "vxbsky.app",
            "bskye" => "bskye.app",
            "bskyx" => "bskyx.app",
            _ => return None,
        };

        Some(format!("https://{}/profile/{}/post/{}", converter_domain, username, post_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_links() {
        let converter = LinkConverter::new();
        
        // Twitter test cases
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

        // Bluesky test cases
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
}