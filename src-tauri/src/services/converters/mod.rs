pub mod twitter;
pub mod bluesky;
pub mod registry;

/// Trait that all link converters must implement
pub trait LinkConverterStrategy: Send + Sync {
    /// Returns the platform name this converter handles (e.g., "twitter", "bluesky")
    fn platform_name(&self) -> &'static str;
    
    /// Checks if the given URL matches this platform's URL pattern
    fn matches(&self, url: &str) -> bool;
    
    /// Converts a URL to the specified converter format
    /// Returns None if the URL is already converted or invalid
    fn convert(&self, url: &str, converter: &str) -> Option<String>;
    
    /// Returns available converter options for this platform
    fn available_converters(&self) -> Vec<&'static str>;
}
