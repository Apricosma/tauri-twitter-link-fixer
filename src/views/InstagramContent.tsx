import ServiceView from "../components/media-service/ServiceView";

const InstagramContent = () => {
  return (
    <ServiceView
      platform="instagram"
      title="Instagram"
      icon="📸"
      description="Instagram link conversion coming soon"
      howItWorksSteps={[
        "Instagram post link conversion",
        "Story link handling", 
        "Multiple converter options",
        "Automatic clipboard detection"
      ]}
      status="coming-soon"
    />
  );
};

export default InstagramContent;