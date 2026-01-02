import ServiceView from "../components/media-service/ServiceView";

const BlueSkyContent = () => {
  return (
    <ServiceView
      platform="bluesky"
      title="BlueSky"
      icon="🦋"
      description="Convert BlueSky post links"
      howItWorksSteps={[
        "Copy any BlueSky post link (bsky.app)",
        "The link will be automatically converted to your selected format",
        "Converted link replaces the original in your clipboard",
        "Paste your converted link into Discord, etc"
      ]}
      status="available"
    />
  );
};

export default BlueSkyContent;