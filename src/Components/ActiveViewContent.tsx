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
  const ActiveComponent = viewRegistry[activeView];
  return (
    <div className="flex-1 p-2">
      {/* {activeView === "home" && <HomeContent />}
      {activeView === "twitter" && <TwitterContent />} */}
      <ActiveComponent />
    </div>
  );
};

export default ActiveViewContent;
