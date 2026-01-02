import { usePlatform } from "../hooks/usePlatform";
import ToggleSwitch from "../components/ToggleSwitch";
import DropdownMenu from "../components/DropdownMenu";
import ConversionNotification from "../components/ConversionNotification";

const TwitterContent = () => {
  const {
    platformData,
    lastConversion,
    showNotification,
    handleToggle,
    handleDropdownSelect,
  } = usePlatform("twitter");

  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center space-x-3">
        <span className="text-3xl">🐦</span>
        <div>
          <h1 className="text-2xl font-bold">Twitter / X</h1>
          <p className="text-muted-foreground">
            Convert Twitter and X links to embeddable formats
          </p>
        </div>
      </div>

      {platformData ? (
        <div className="space-y-6">
          <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
            <h3 className="font-semibold mb-4">Platform Settings</h3>
            <div className="space-y-4">
              <ToggleSwitch
                id="toggle-twitter"
                label="Enable Twitter link conversion"
                platform="twitter"
                initialChecked={platformData.enabled}
                onToggle={handleToggle}
              />

              {platformData.enabled && (
                <div className="pt-4 border-t">
                  <label className="text-sm font-medium mb-2 block">
                    Converter Service
                  </label>
                  <DropdownMenu
                    label="Select converter"
                    options={platformData.converters}
                    platform="twitter"
                    selected={platformData.selected}
                    onSelect={handleDropdownSelect}
                  />
                </div>
              )}
            </div>
          </div>

          <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
            <h3 className="font-semibold mb-3">How it works</h3>
            <div className="space-y-2 text-sm text-muted-foreground">
              <p>• Copy any Twitter or X link (twitter.com or x.com)</p>
              <p>• The link will be automatically converted to your selected format</p>
              <p>• Converted link replaces the original in your clipboard</p>
              <p>• Perfect for sharing embeddable content in Discord, etc.</p>
            </div>
          </div>
        </div>
      ) : (
        <div className="flex items-center justify-center py-8">
          <div className="flex items-center space-x-2">
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-primary"></div>
            <span className="text-muted-foreground">Loading...</span>
          </div>
        </div>
      )}

      <ConversionNotification show={showNotification} conversion={lastConversion} />
    </div>
  );
};

export default TwitterContent;