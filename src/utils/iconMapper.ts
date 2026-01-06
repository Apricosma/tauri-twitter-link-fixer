import * as SimpleIcons from "@icons-pack/react-simple-icons";
import { HelpCircle } from "lucide-react";


const ICON_MAPPING: Record<string, string> = {
  'twitter': 'SiX', 
  'x': 'SiX',
  'bluesky': 'SiBluesky',
  'tiktok': 'SiTiktok',
  'instagram': 'SiInstagram',
};

/**
 * Dynamically loads a Simple Icons component based on platform identifier
 * @param iconIdentifier - The icon identifier from the config (e.g., "x", "bluesky", "tiktok")
 * @returns React component for the icon, or HelpCircle as fallback
 */
export function getSimpleIcon(iconIdentifier: string): React.ComponentType<any> {
  const icons = SimpleIcons as any;
  
  // Check manual mapping first
  const mappedKey = ICON_MAPPING[iconIdentifier.toLowerCase()];
  if (mappedKey && icons[mappedKey]) {
    return icons[mappedKey];
  }
  
  // Fallback to automatic mapping
  // Simple icons use "Si" prefix followed by capitalized name
  // e.g., "twitter" -> "SiTwitter", "x" -> "SiX", "bluesky" -> "SiBluesky"
  const iconKey = `Si${iconIdentifier.charAt(0).toUpperCase()}${iconIdentifier.slice(1).toLowerCase()}`;
  
  // Return the icon if found, otherwise return a fallback
  return icons[iconKey] || HelpCircle;
}
