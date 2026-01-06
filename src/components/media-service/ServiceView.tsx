import { usePlatform } from "../../hooks/usePlatform";
import ServiceHeader from "./ServiceHeader";
import PlatformSettings from "./PlatformSettings";
import HowItWorksSection from "./HowItWorksSection";
import LoadingSpinner from "../LoadingSpinner";
import ComingSoonView from "./ComingSoonView";
import ConversionNotification from "../ConversionNotification";

interface ServiceViewProps {
  platform: string;
  title: string;
  description: string;
  howItWorksSteps: string[];
  status?: "available" | "coming-soon";
}

const ServiceView: React.FC<ServiceViewProps> = ({
  platform,
  title,
  description,
  howItWorksSteps,
  status = "available",
}) => {
  const {
    platformData,
    lastConversion,
    showNotification,
    handleToggle,
    handleDropdownSelect,
  } = usePlatform(platform);

  if (status === "coming-soon") {
    return (
      <ComingSoonView title={title} description={description} />
    );
  }

  return (
    <div className="space-y-6 p-6 bg-sidebar rounded-2xl min-h-full rounded-bl-none rounded-br-none">
      <ServiceHeader title={title} description={description} />

      {platformData ? (
        <div className="space-y-6">
          <PlatformSettings
            platform={platform}
            title={title}
            enabled={platformData.enabled}
            converters={platformData.converters}
            selected={platformData.selected || ""}
            onToggle={(_, enabled) => handleToggle(enabled)}
            onDropdownSelect={handleDropdownSelect}
          />

          <HowItWorksSection steps={howItWorksSteps} />
        </div>
      ) : (
        <LoadingSpinner />
      )}

      <ConversionNotification
        show={showNotification}
        conversion={lastConversion}
      />
    </div>
  );
};

export default ServiceView;
