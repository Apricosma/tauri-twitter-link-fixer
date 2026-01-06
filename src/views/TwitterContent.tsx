import ServiceView from "../components/media-service/ServiceView";

const TwitterContent = () => {
  return (
    <ServiceView
      platform="twitter"
      title="Twitter / X"
      icon="twitter"
      description="Convert Twitter and X links"
      howItWorksSteps={[
        "Copy any Twitter or X link (twitter.com or x.com)",
        "The link will be automatically converted to your selected format",
        "Converted link replaces the original in your clipboard",
        "Paste your converted link into Discord, etc"
      ]}
      status="available"
    />
  );
};

export default TwitterContent;