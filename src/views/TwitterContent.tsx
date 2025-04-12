import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import ContentContainer from "../Components/ContentContainer";
import ToggleSwitch from "../Components/ToggleSwitch";
import DropdownMenu from "../Components/DropdownMenu";

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

const TwitterContent = () => {
  const [config, setConfig] = useState<SourcesConfig | null>(null);

  // Set up event listener and fetch initial state
  useEffect(() => {
    // Fetch initial state
    invoke<SourcesConfig>("get_state")
      .then((data) => setConfig(data))
      .catch(console.error);

    // Listen for state changes
    const unlisten = listen<SourcesConfig>("state-changed", (event) => {
      setConfig(event.payload);
    });

    // Cleanup listener on unmount
    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const handleToggle = async (enabled: boolean) => {
    if (!config) return;

    try {
      await invoke("toggle_platform", { platform: "twitter", enabled });
      // State will be updated via event listener
    } catch (error) {
      console.error("Failed to toggle platform:", error);
    }
  };

  const handleDropdownSelect = async (selected: string) => {
    if (!config) return;

    try {
      await invoke("select_converter", {
        platform: "twitter",
        converterName: selected,
      });
      // State will be updated via event listener
    } catch (error) {
      console.error("Failed to select converter:", error);
    }
  };

  const twitter = config?.sources.find((s) => s.platform === "twitter");

  return (
    <>
      <h1 className="text-4xl flex items-center justify-center border-b-1 border-gray-700/50 pb-2">
        Twitter Embeds
      </h1>

      <ContentContainer>
        {twitter && (
          <>
            <ToggleSwitch
              id="toggle-twitter"
              label="Enable"
              platform="twitter"
              initialChecked={twitter.data.enabled}
              onToggle={handleToggle}
            />
            <DropdownMenu
              label="Converter Source"
              options={twitter.data.converters}
              platform="twitter"
              selected={twitter.data.selected}
              onSelect={handleDropdownSelect}
            />
          </>
        )}
      </ContentContainer>
    </>
  );
};

export default TwitterContent;