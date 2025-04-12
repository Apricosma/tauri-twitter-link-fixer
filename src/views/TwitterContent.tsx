import { usePlatform } from "../hooks/usePlatform";
import ContentContainer from "../Components/ContentContainer";
import ToggleSwitch from "../Components/ToggleSwitch";
import DropdownMenu from "../Components/DropdownMenu";
import ConversionNotification from "../Components/ConversionNotification";

const TwitterContent = () => {
  const {
    platformData,
    lastConversion,
    showNotification,
    handleToggle,
    handleDropdownSelect,
  } = usePlatform("twitter");

  return (
    <>
      <h1 className="text-4xl flex items-center justify-center border-b-1 border-gray-700/50 pb-2">
        Twitter Embeds
      </h1>

      <ContentContainer>
        {platformData && (
          <>
            <ToggleSwitch
              id="toggle-twitter"
              label="Enable"
              platform="twitter"
              initialChecked={platformData.enabled}
              onToggle={handleToggle}
            />
            <DropdownMenu
              label="Converter Source"
              options={platformData.converters}
              platform="twitter"
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

export default TwitterContent;