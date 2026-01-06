export interface PlatformMetadata {
  title: string;
  icon: string; // Simple Icons identifier (e.g., "x", "bluesky", "tiktok")
}

export interface ConverterConfig {
  enabled: boolean;
  converters: string[];
  selected: string;
}

export interface PlatformSource {
  platform: string;
  metadata: PlatformMetadata;
  data: ConverterConfig;
}

export interface AppConfig {
  sources: PlatformSource[];
}
