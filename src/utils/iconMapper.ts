import { HelpCircle } from "lucide-react";
import { lazy, ComponentType } from "react";
import { SiX, SiBluesky, SiTiktok, SiInstagram } from "@icons-pack/react-simple-icons";

/**
 * Mapping of platform identifiers to their corresponding Simple Icons component names.
 * This allows for custom mappings when the platform name doesn't directly match the icon name.
 * 
 * @example
 * 'twitter' and 'x' both map to 'SiX' since Twitter rebranded to X
 */
const ICON_MAPPING: Record<string, string> = {
  'twitter': 'SiX', 
  'x': 'SiX',
  'bluesky': 'SiBluesky',
  'tiktok': 'SiTiktok',
  'instagram': 'SiInstagram',
};

/**
 * Preloaded icons for common platforms to avoid lazy loading delays.
 * These icons are eagerly loaded at startup for instant rendering.
 */
const PRELOADED_ICONS: Record<string, ComponentType<any>> = {
  'SiX': SiX,
  'SiBluesky': SiBluesky,
  'SiTiktok': SiTiktok,
  'SiInstagram': SiInstagram,
};

/**
 * Dynamically loads a Simple Icons component based on platform identifier.
 * Common platform icons are preloaded for instant rendering, while others
 * use React.lazy to defer loading until needed.
 * 
 * The function first checks the ICON_MAPPING for manual mappings, then falls back
 * to automatic mapping using the "Si" prefix convention (e.g., "bluesky" -> "SiBluesky").
 * 
 * @param iconIdentifier - The icon identifier from the config (e.g., "x", "bluesky", "tiktok")
 * @returns A React component for the icon, or HelpCircle as fallback
 * 
 * @example
 * ```tsx
 * const Icon = getSimpleIcon('twitter');
 * // Returns preloaded SiX component (instant render)
 * 
 * <Suspense fallback={<div>Loading...</div>}>
 *   <Icon />
 * </Suspense>
 * ```
 * 
 * @remarks
 * Preloaded icons (Twitter, Bluesky, TikTok, Instagram) render instantly without lazy loading.
 * Other icons still require a React Suspense boundary for lazy loading.
 * If the icon is not found, it falls back to the HelpCircle icon from lucide-react.
 */
export function getSimpleIcon(iconIdentifier: string): React.ComponentType<any> {
  // Check manual mapping first
  const mappedKey = ICON_MAPPING[iconIdentifier.toLowerCase()];
  
  // Fallback to automatic mapping if no manual mapping exists
  // Simple icons use "Si" prefix followed by capitalized name
  // e.g., "twitter" -> "SiTwitter", "x" -> "SiX", "bluesky" -> "SiBluesky"
  const iconKey = mappedKey || `Si${iconIdentifier.charAt(0).toUpperCase()}${iconIdentifier.slice(1).toLowerCase()}`;
  
  // Check if icon is preloaded for instant rendering
  if (PRELOADED_ICONS[iconKey]) {
    return PRELOADED_ICONS[iconKey];
  }
  
  // Return a lazy-loaded component for non-preloaded icons
  return lazy(async () => {
    try {
      // Dynamically import the entire icons package
      const icons = await import("@icons-pack/react-simple-icons");
      
      // Extract the specific icon from the module
      const IconComponent = icons[iconKey as keyof typeof icons];
      
      // If icon exists, return it wrapped in default export for lazy loading
      if (IconComponent) {
        return { default: IconComponent as React.ComponentType<any> };
      }
      
      // Fallback to HelpCircle if icon not found
      return { default: HelpCircle };
    } catch (error) {
      // If import fails, return HelpCircle as fallback
      console.error(`Failed to load icon for "${iconIdentifier}":`, error);
      return { default: HelpCircle };
    }
  });
}
