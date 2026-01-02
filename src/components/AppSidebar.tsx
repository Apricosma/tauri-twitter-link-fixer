import { Home, Twitter, Cloud, Instagram, Music, Settings } from "lucide-react";
import { ViewType } from "./ActiveViewContent";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarHeader,
  SidebarFooter,
} from "./ui/sidebar";

interface AppSidebarProps {
  activeView: ViewType;
  setActiveView: (view: ViewType) => void;
}

// Home item separated from platforms
const homeItem = {
  title: "Home",
  icon: Home,
  view: ViewType.Home,
};

// Platform items without Home
const platformItems = [
  {
    title: "Twitter",
    icon: Twitter,
    view: ViewType.Twitter,
  },
  {
    title: "BlueSky",
    icon: Cloud,
    view: ViewType.BlueSky,
  },
  {
    title: "Instagram",
    icon: Instagram,
    view: ViewType.Instagram,
  },
  {
    title: "TikTok",
    icon: Music,
    view: ViewType.TikTok,
  },
];

export function AppSidebar({ activeView, setActiveView }: AppSidebarProps) {
  return (
    <Sidebar>
      <SidebarHeader>
        <div className="flex items-center gap-2 px-2 py-2">
          <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
            <span className="text-lg">🔗</span>
          </div>
          <div className="flex flex-col gap-0.5 leading-none">
            <span className="font-semibold">Link Fixer</span>
            <span className="text-xs text-muted-foreground">Social Media Converter</span>
          </div>
        </div>
      </SidebarHeader>
      
      <SidebarContent className="gap-0">
        <SidebarGroup className="pb-0">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton 
                  onClick={() => setActiveView(homeItem.view)}
                  isActive={activeView === homeItem.view}
                  tooltip={homeItem.title}
                >
                  <homeItem.icon />
                  <span>{homeItem.title}</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Platforms</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {platformItems.map((item) => {
                const isActive = activeView === item.view;
                return (
                  <SidebarMenuItem key={item.title}>
                    <SidebarMenuButton 
                      onClick={() => setActiveView(item.view)}
                      isActive={isActive}
                      tooltip={item.title}
                    >
                      <item.icon />
                      <span>{item.title}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                );
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      
      <SidebarFooter>
        <div className="p-2">
          <div className="flex items-center justify-between text-xs text-muted-foreground">
            <span>v1.0.0</span>
            <Settings className="size-4" />            
          </div>
        </div>
      </SidebarFooter>
    </Sidebar>
  );
}