import ServiceView from "../components/media-service/ServiceView";

const TikTokContent = () => {
  return (
    <ServiceView
      platform="tiktok"
      title="TikTok"
      icon="🎵"
      description="TikTok link conversion coming soon"
      howItWorksSteps={[
        "TikTok video link conversion", 
        "Profile link handling",
        "Multiple converter options",
        "Automatic clipboard detection"
      ]}
      status="coming-soon"
    />
  );
};

export default TikTokContent;