import { useEffect, useState } from "react";
import sources from "../converter-sources.json";
import Card from "../Components/Card";
import ContentContainer from "../Components/ContentContainer";
import { invoke } from "@tauri-apps/api/core";
import ToggleSwitch from "../Components/ToggleSwitch";
import DropdownMenu from "../Components/DropdownMenu";

const TwitterContent = () => {
  const twitterSources = sources.sources.find(
    (source) => source.platform === "twitter"
  );
  const twitterEmbeds = twitterSources?.embeds || [];

  const [toggleEnabled, setToggleEnabled] = useState(false);
  const [selectedConverter, setSelectedConverter] = useState<string | null>(
    null
  );

  const handleToggle = (checked: boolean) => {
    setToggleEnabled(checked);
  };

  // Handle dropdown selection changes
  const handleDropdownSelect = (selected: string) => {
    setSelectedConverter(selected);
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
        <DropdownMenu label="Converter Source" options={twitterEmbeds} onSelect={handleDropdownSelect} selected={selectedConverter} />
      </ContentContainer>
      {/* <div className="flex justify-center">
        <div className="border-b-1 border-gray-700/50 w-3/4"></div>
      </div> */}
    </>
  );
};

export default TwitterContent;
