import { useEffect, useState } from "react";
import TwitterContent from "../views/TwitterContent";
import HomeContent from "../views/HomeContent";
import BlueSkyContent from "../views/BlueSkyContent";
import InstagramContent from "../views/InstagramContent";
import TikTokContent from "../views/TikTokContent";

interface ActiveViewContentProps {
  activeView: ViewType;
}

export enum ViewType {
  Home = "home",
  Twitter = "twitter",
  BlueSky = "bluesky",
  Instagram = "instagram",
  TikTok = "tiktok",
}

interface ViewTypeMap {
  [ViewType.Home]: React.FC;
  [ViewType.Twitter]: React.FC;
  [ViewType.BlueSky]: React.FC;
  [ViewType.Instagram]: React.FC;
  [ViewType.TikTok]: React.FC;
}

const viewRegistry: ViewTypeMap = {
  [ViewType.Home]: HomeContent,
  [ViewType.Twitter]: TwitterContent,
  [ViewType.BlueSky]: BlueSkyContent,
  [ViewType.Instagram]: InstagramContent,
  [ViewType.TikTok]: TikTokContent,
};

const ActiveViewContent = ({ activeView }: ActiveViewContentProps) => {
  const [isTransitioning, setIsTransitioning] = useState(false);
  const ActiveComponent = viewRegistry[activeView];

  useEffect(() => {
    setIsTransitioning(true);
    const timer = setTimeout(() => setIsTransitioning(false), 150);
    return () => clearTimeout(timer);
  }, [activeView]);

  return (
    <div className="flex-1 rounded-lg border bg-background/50 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className={`transition-opacity duration-150 ${
        isTransitioning ? 'opacity-0' : 'opacity-100'
      }`}>
        <ActiveComponent />
      </div>
    </div>
  );
};

export default ActiveViewContent;
