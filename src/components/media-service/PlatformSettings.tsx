import ToggleSwitch from "../ToggleSwitch";
import DropdownMenu from "../DropdownMenu";

interface PlatformSettingsProps {
  platform: string;
  title: string;
  enabled: boolean;
  converters: any[];
  selected: string;
  onToggle: (platform: string, enabled: boolean) => void;
  onDropdownSelect: (platform: string, selected: string) => void;
}

const PlatformSettings: React.FC<PlatformSettingsProps> = ({
  platform,
  title,
  enabled,
  converters,
  selected,
  onToggle,
  onDropdownSelect,
}) => {
  return (
    <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
      <h3 className="font-semibold mb-4">Platform Settings</h3>
      <div className="space-y-4">
        <ToggleSwitch
          id={`toggle-${platform}`}
          label={`Enable ${title} link conversion`}
          platform={platform}
          initialChecked={enabled}
          onToggle={onToggle}
        />

        {enabled && (
          <div className="pt-4 border-t">
            <label className="text-sm font-medium mb-2 block">
              Converter Service
            </label>
            <DropdownMenu
              label="Select converter"
              options={converters}
              platform={platform}
              selected={selected}
              onSelect={onDropdownSelect}
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default PlatformSettings;