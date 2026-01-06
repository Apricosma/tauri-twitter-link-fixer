import ServiceView from "../components/media-service/ServiceView";

const TikTokContent = () => {
  return (
    <ServiceView
      platform="tiktok"
      title="TikTok"
      description="Convert TikTok video links"
      howItWorksSteps={[
        "Copy any TikTok video link (tiktok.com/@username/video/id)",
        "The link will be automatically converted to your selected format",
        "Converted link replaces the original in your clipboard",
        "Paste your converted link into Discord, etc"
      ]}
      status="available"
    />
  );
};

export default TikTokContent;