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

interface ConversionEvent {
  original: string;
  converted: string;
}

const TwitterContent = () => {
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
      await invoke("toggle_platform", { platform: "twitter", enabled });
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
            
            {/* Notification toast */}
            {showNotification && lastConversion && (
              <div className="fixed bottom-4 right-4 bg-appforeground text-gray-100 p-4 rounded-lg shadow-lg transition-opacity duration-300 max-w-2xl border border-gray-700">
                <div className="font-bold mb-2 text-red-400">Link Converted!</div>
                <div className="text-sm opacity-90 space-y-2">
                  <div>
                    <div className="font-semibold mb-1">From ({new URL(lastConversion.original).hostname}):</div>
                    <div className="bg-appbg/50 p-2 rounded break-all">
                      {lastConversion.original}
                    </div>
                  </div>
                  <div>
                    <div className="font-semibold mb-1">To ({new URL(lastConversion.converted).hostname}):</div>
                    <div className="bg-appbg/50 p-2 rounded break-all">
                      {lastConversion.converted}
                    </div>
                  </div>
                </div>
              </div>
            )}
          </>
        )}
      </ContentContainer>
    </>
  );
};

export default TwitterContent;