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
  const [toggleEnabled, setToggleEnabled] = useState(false);
  const [selectedConverter, setSelectedConverter] = useState<string | null>(null);
  const [twitterEmbeds, setTwitterEmbeds] = useState<string[]>([]);

  useEffect(() => {
    // Fetch config from backend
    invoke<SourcesConfig>("get_config")
      .then((config) => {
        const twitter = config.sources.find((source) => source.platform === "twitter");
        if (twitter) {
          setToggleEnabled(twitter.data.enabled);
          setSelectedConverter(twitter.data.selected);
          setTwitterEmbeds(twitter.data.converters);
        }
      })
      .catch((error) => {
        console.error("Failed to load config", error);
      });
  }, []);

  const handleToggle = (checked: boolean) => {
    setToggleEnabled(checked);

    // Update backend immediately
    invoke("update_config", {
      platform: "twitter",
      enabled: checked,
      selectedConverter: selectedConverter,
    }).catch((error) => {
      console.error("Failed to update config:", error);
    });
  };

  const handleDropdownSelect = (selected: string) => {
    setSelectedConverter(selected);

    // Update backend immediately
    invoke("update_config", {
      platform: "twitter",
      enabled: toggleEnabled,
      selectedConverter: selected,
    }).catch((error) => {
      console.error("Failed to update config:", error);
    });
  };

  return (
    <>
      <h1 className="text-4xl flex items-center justify-center border-b-1 border-gray-700/50 pb-2">
        Twitter Embeds
      </h1>

      <ContentContainer>
        <ToggleSwitch
          id="toggle-twitter"
          label="Enable"
          checked={toggleEnabled}
          onChange={handleToggle}
        />
        <DropdownMenu
          label="Converter Source"
          options={twitterEmbeds}
          onSelect={handleDropdownSelect}
          selected={selectedConverter}
        />
      </ContentContainer>
    </>
  );
};

export default TwitterContent;
