import { Switch } from "../ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import { Label } from "../ui/label";

interface PlatformSettingsProps {
  platform: string;
  title: string;
  enabled: boolean;
  converters: any[];
  selected: string;
  onToggle: (platform: string, enabled: boolean) => void;
  onDropdownSelect: (selected: string) => void;
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
    <Card className="gap-2">
      <CardHeader>
        <CardTitle className="border-b-1 pb-4">Platform Settings</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex items-center justify-between space-x-2">
          <Label htmlFor={`switch-${platform}`}>
            Enable {title} link conversion
          </Label>
          <Switch
            id={`switch-${platform}`}
            checked={enabled}
            onCheckedChange={(checked) => onToggle(platform, checked)}
          />
        </div>

        {enabled && (
          <div className="space-y-2 flex justify-between">
            <Label htmlFor={`select-${platform}`}>Converter Service</Label>
            <Select
              value={selected || ""}
              onValueChange={(value) => {
                onDropdownSelect(value);
              }}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select converter" />
              </SelectTrigger>
              <SelectContent>
                {converters.map((converter) => (
                  <SelectItem key={converter} value={converter}>
                    {converter}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}
      </CardContent>
    </Card>
  );
};

export default PlatformSettings;
