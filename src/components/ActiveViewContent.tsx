import HomeContent from "../views/HomeContent";
import ServiceView from "./media-service/ServiceView";
import { useConfig } from "../hooks/useConfig";

interface ActiveViewContentProps {
  activeView: string;
  setActiveView: (view: string) => void;
}

const howItWorksByPlatform: Record<string, string[]> = {
  twitter: [
    "Copy any Twitter or X link (twitter.com or x.com)",
    "The link will be automatically converted to your selected format",
    "Converted link replaces the original in your clipboard",
    "Paste your converted link into Discord, etc",
  ],
  bluesky: [
    "Copy any BlueSky post link (bsky.app)",
    "The link will be automatically converted to your selected format",
    "Converted link replaces the original in your clipboard",
    "Paste your converted link into Discord, etc",
  ],
  tiktok: [
    "Copy any TikTok video link (tiktok.com/@username/video/id)",
    "The link will be automatically converted to your selected format",
    "Converted link replaces the original in your clipboard",
    "Paste your converted link into Discord, etc",
  ],
  instagram: [
    "Copy any Instagram reel link (instagram.com/username/reel/id)",
    "The link will be automatically converted to your selected format",
    "Converted link replaces the original in your clipboard",
    "Paste your converted link into Discord, etc",
  ],
};

// Export ViewType as a type alias for string to maintain compatibility
export type ViewType = string;

const ActiveViewContent = ({
  activeView,
  setActiveView,
}: ActiveViewContentProps) => {
  const { config, loading } = useConfig();

  if (loading || !config) {
    return (
      <div className="flex-1 h-full rounded-lg border bg-background/50 backdrop-blur supports-[backdrop-filter]:bg-background/60 overflow-hidden border-b-0 rounded-bl-none rounded-br-none">
        <div className="flex items-center justify-center h-full">
          <div className="text-muted-foreground">Loading...</div>
        </div>
      </div>
    );
  }

  // Home view
  if (activeView === "home") {
    return (
      <div className="flex-1 h-full rounded-lg border bg-background/50 backdrop-blur supports-[backdrop-filter]:bg-background/60 overflow-hidden border-b-0 rounded-bl-none rounded-br-none">
        <HomeContent setActiveView={setActiveView} />
      </div>
    );
  }

  // Find platform in config
  const platformSource = config.sources.find((s) => s.platform === activeView);
  
  if (!platformSource) {
    return (
      <div className="flex-1 h-full rounded-lg border bg-background/50 backdrop-blur supports-[backdrop-filter]:bg-background/60 overflow-hidden border-b-0 rounded-bl-none rounded-br-none">
        <div className="flex items-center justify-center h-full">
          <div className="text-muted-foreground">Platform not found</div>
        </div>
      </div>
    );
  }

  const platform = platformSource.platform;

  const defaultSteps = [
    `Copy any ${platformSource.metadata.title} link`,
    "The link will be automatically converted to your selected format",
    "Converted link replaces the original in your clipboard",
    "Paste your converted link into Discord, etc",
  ];

  // Dynamic platform view
  return (
    <div className="flex-1 h-full rounded-lg border bg-background/50 backdrop-blur supports-[backdrop-filter]:bg-background/60 overflow-hidden border-b-0 rounded-bl-none rounded-br-none">
      <ServiceView
        platform={platform}
        title={platformSource.metadata.title}
        description={`Convert ${platformSource.metadata.title} links`}
        howItWorksSteps={howItWorksByPlatform[platform] ?? defaultSteps}
        status="available"
      />
    </div>
  );
};

export default ActiveViewContent;
