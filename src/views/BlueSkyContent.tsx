import ServiceView from "../components/media-service/ServiceView";

const BlueSkyContent = () => {
  return (
    <ServiceView
      platform="bluesky"
      title="BlueSky"
      icon="🦋"
      description="Convert BlueSky post links to embeddable formats for better sharing and previews"
      howItWorksSteps={[
        "Copy any BlueSky post link (bsky.app)",
        "The link will be automatically converted to your selected format",
        "Converted link replaces the original in your clipboard",
        "Enhanced previews and embeddability for social sharing"
      ]}
      status="available"
    />
  );
};

export default BlueSkyContent;