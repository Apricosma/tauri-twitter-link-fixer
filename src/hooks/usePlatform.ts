import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface SourcesConfig {
  sources: PlatformSource[];
}

interface PlatformSource {
  platform: string;
  data: {
    enabled: boolean;
    converters: string[];
    selected: string | null;
  };
}

interface ConversionEvent {
  original: string;
  converted: string;
}

export function usePlatform(platformName: string) {
  const [config, setConfig] = useState<SourcesConfig | null>(null);
  const [lastConversion, setLastConversion] = useState<ConversionEvent | null>(null);
  const [showNotification, setShowNotification] = useState(false);

  useEffect(() => {
    // Fetch initial state
    invoke<SourcesConfig>("get_state")
      .then((data) => setConfig(data))
      .catch(console.error);

    // Start clipboard monitoring
    invoke("start_clipboard_monitor").catch(console.error);

    // Listen for state changes
    const stateUnlisten = listen<SourcesConfig>("state-changed", (event) => {
      setConfig(event.payload);
    });

    // Listen for link conversions
    const conversionUnlisten = listen<ConversionEvent>("link-converted", (event) => {
      setLastConversion(event.payload);
      setShowNotification(true);
      setTimeout(() => setShowNotification(false), 3000); // Hide after 3 seconds
    });

    // Cleanup listeners on unmount
    return () => {
      stateUnlisten.then(fn => fn());
      conversionUnlisten.then(fn => fn());
    };
  }, []);

  const handleToggle = async (enabled: boolean) => {
    if (!config) return;

    try {
      await invoke("toggle_platform", { platform: platformName, enabled });
    } catch (error) {
      console.error("Failed to toggle platform:", error);
    }
  };

  const handleDropdownSelect = async (selected: string) => {
    if (!config) return;

    try {
      await invoke("select_converter", {
        platform: platformName,
        converterName: selected,
      });
    } catch (error) {
      console.error("Failed to select converter:", error);
    }
  };

  const platformData = config?.sources.find((s) => s.platform === platformName)?.data;

  return {
    config,
    platformData,
    lastConversion,
    showNotification,
    handleToggle,
    handleDropdownSelect,
  };
}