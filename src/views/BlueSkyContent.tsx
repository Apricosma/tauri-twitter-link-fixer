import { usePlatform } from "../hooks/usePlatform";
import ContentContainer from "../Components/ContentContainer";
import ToggleSwitch from "../Components/ToggleSwitch";
import DropdownMenu from "../Components/DropdownMenu";
import ConversionNotification from "../Components/ConversionNotification";

const BlueSkyContent = () => {
  const {
    platformData,
    lastConversion,
    showNotification,
    handleToggle,
    handleDropdownSelect,
  } = usePlatform("bluesky");

  return (
    <>
      <h1 className="text-4xl flex items-center justify-center border-b-1 border-gray-700/50 pb-2">
        BlueSky Embeds
      </h1>

      <ContentContainer>
        {platformData && (
          <>
            <ToggleSwitch
              id="toggle-bluesky"
              label="Enable"
              platform="bluesky"
              initialChecked={platformData.enabled}
              onToggle={handleToggle}
            />
            <DropdownMenu
              label="Converter Source"
              options={platformData.converters}
              platform="bluesky"
              selected={platformData.selected}
              onSelect={handleDropdownSelect}
            />
            
            <ConversionNotification show={showNotification} conversion={lastConversion} />
          </>
        )}
      </ContentContainer>
    </>
  );
};

export default BlueSkyContent;