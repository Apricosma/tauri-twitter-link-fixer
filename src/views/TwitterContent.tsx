import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
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

  // Fetch from backend on mount
  useEffect(() => {
    invoke<SourcesConfig>("get_state")
      .then((data) => setConfig(data))
      .catch(console.error);
  }, []);

  const handleToggle = async (enabled: boolean) => {
    if (!config) return;

    await invoke("toggle_platform", { platform: "twitter", enabled });
    const updated = await invoke<SourcesConfig>("get_state");
    setConfig(updated);
  };

  const handleDropdownSelect = async (selected: string) => {
    if (!config) return;

    try {
      
      await invoke("select_converter", {
        platform: "twitter",
        converterName: selected,
      });
      const updated = await invoke<SourcesConfig>("get_state");
      setConfig(updated);
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