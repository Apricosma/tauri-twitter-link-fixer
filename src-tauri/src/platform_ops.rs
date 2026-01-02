use crate::config::app_config::{Platform, PlatformConverters, PlatformSource, SourcesConfig};
use crate::services::link_converter::LinkConverter;

/// Generic platform operations trait for common platform functionality
pub trait PlatformOperations {
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn get_selected_converter(&self) -> Option<String>;
    fn set_converter_by_name(&mut self, converter_name: &str) -> bool;
    fn try_convert_link(&self, link_converter: &LinkConverter, url: &str, platform_name: &str) -> Option<String>;
}

impl PlatformOperations for PlatformConverters<crate::config::app_config::TwitterConverters> {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_selected_converter(&self) -> Option<String> {
        self.selected.as_ref().and_then(|c| {
            serde_json::to_string(c).ok().map(|s| s.trim_matches('"').to_string())
        })
    }

    fn set_converter_by_name(&mut self, converter_name: &str) -> bool {
        if let Some(found) = self.converters.iter().find(|c| {
            let serialized = serde_json::to_string(c)
                .unwrap_or_default()
                .trim_matches('"')
                .to_string();
            serialized == converter_name
        }) {
            self.selected = Some(found.clone());
            true
        } else {
            false
        }
    }

    fn try_convert_link(&self, link_converter: &LinkConverter, url: &str, platform_name: &str) -> Option<String> {
        if self.enabled {
            if let Some(selected) = &self.selected {
                let converter_name = serde_json::to_string(selected)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();
                return link_converter.convert_link(url, platform_name, &converter_name);
            }
        }
        None
    }
}

impl PlatformOperations for PlatformConverters<crate::config::app_config::BlueskyConverters> {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_selected_converter(&self) -> Option<String> {
        self.selected.as_ref().and_then(|c| {
            serde_json::to_string(c).ok().map(|s| s.trim_matches('"').to_string())
        })
    }

    fn set_converter_by_name(&mut self, converter_name: &str) -> bool {
        if let Some(found) = self.converters.iter().find(|c| {
            let serialized = serde_json::to_string(c)
                .unwrap_or_default()
                .trim_matches('"')
                .to_string();
            serialized == converter_name
        }) {
            self.selected = Some(found.clone());
            true
        } else {
            false
        }
    }

    fn try_convert_link(&self, link_converter: &LinkConverter, url: &str, platform_name: &str) -> Option<String> {
        if self.enabled {
            if let Some(selected) = &self.selected {
                let converter_name = serde_json::to_string(selected)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();
                return link_converter.convert_link(url, platform_name, &converter_name);
            }
        }
        None
    }
}

// Helper methods for PlatformSource enum
impl PlatformSource {
    pub fn get_platform_type(&self) -> Platform {
        match self {
            PlatformSource::Twitter(_) => Platform::Twitter,
            PlatformSource::Bluesky(_) => Platform::Bluesky,
        }
    }

    pub fn get_platform_name(&self) -> &'static str {
        match self {
            PlatformSource::Twitter(_) => "twitter",
            PlatformSource::Bluesky(_) => "bluesky",
        }
    }

    pub fn get_operations_mut(&mut self) -> &mut dyn PlatformOperations {
        match self {
            PlatformSource::Twitter(data) => data,
            PlatformSource::Bluesky(data) => data,
        }
    }

    pub fn get_operations(&self) -> &dyn PlatformOperations {
        match self {
            PlatformSource::Twitter(data) => data,
            PlatformSource::Bluesky(data) => data,
        }
    }

    pub fn try_convert_link(&self, link_converter: &LinkConverter, url: &str) -> Option<String> {
        self.get_operations().try_convert_link(link_converter, url, self.get_platform_name())
    }
}

/// Parse platform string into Platform enum
pub fn parse_platform(platform_str: &str) -> Option<Platform> {
    match platform_str.to_lowercase().as_str() {
        "twitter" => Some(Platform::Twitter),
        "bluesky" => Some(Platform::Bluesky),
        _ => None,
    }
}

/// Execute operation on platform data if platform exists
pub fn with_platform_data<F, R>(state: &mut SourcesConfig, platform: Platform, operation: F) -> Option<R>
where
    F: FnOnce(&mut dyn PlatformOperations) -> R,
{
    state.sources
        .iter_mut()
        .find(|source| source.get_platform_type() == platform)
        .map(|source| operation(source.get_operations_mut()))
}

/// Try to convert link using any available platform
pub fn try_convert_with_all_platforms(state: &SourcesConfig, link_converter: &LinkConverter, url: &str) -> Option<String> {
    state.sources
        .iter()
        .find_map(|source| source.try_convert_link(link_converter, url))
}