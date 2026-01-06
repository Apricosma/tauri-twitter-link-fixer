import { Suspense, useMemo } from "react";
import { Home, Settings, HelpCircle } from "lucide-react";
import { ViewType } from "./ActiveViewContent";
import { getSimpleIcon } from "../utils/iconMapper";
import { useConfig } from "../hooks/useConfig";
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

export function AppSidebar({ activeView, setActiveView }: AppSidebarProps) {
  const { config, loading, error } = useConfig();

  // this prevents recalculating icon components on every render
  const iconComponents = useMemo(() => {
    if (!config) return new Map();

    return new Map(
      config.sources.map((source) => [
        source.platform,
        getSimpleIcon(source.metadata.icon),
      ])
    );
  }, [config]);

  return (
    <Sidebar>
      <SidebarHeader>
        <div className="flex items-center gap-2 px-2 py-2">
          <div className="flex aspect-square size-8 items-center justify-center rounded-lg text-primary-foreground">
            <span className="text-lg">
              <img src="/appLogo.png" alt="App Logo" className="h-8 w-8" />
            </span>
          </div>
          <div className="flex flex-col gap-0.5 leading-none">
            <span className="font-semibold">Cosma Link Converter</span>
            <span className="text-xs text-muted-foreground">
              Embeds from Various Platforms
            </span>
          </div>
        </div>
      </SidebarHeader>

      <SidebarContent className="gap-0">
        <SidebarGroup className="pb-0">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => setActiveView("home")}
                  isActive={activeView === "home"}
                  tooltip="Home"
                >
                  <Home />
                  <span>Home</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Platforms</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {loading ? (
                <SidebarMenuItem>
                  <div className="px-2 py-1.5 text-sm text-muted-foreground">
                    Loading...
                  </div>
                </SidebarMenuItem>
              ) : error ? (
                <SidebarMenuItem>
                  <div className="px-2 py-1.5 text-sm text-destructive">
                    Error loading platforms
                  </div>
                </SidebarMenuItem>
              ) : config ? (
                config.sources.map((source) => {
                  const Icon =
                    iconComponents.get(source.platform) ||
                    (() => <HelpCircle />);
                  const view = source.platform;
                  const isActive = activeView === view;

                  return (
                    <SidebarMenuItem key={source.platform}>
                      <SidebarMenuButton
                        onClick={() => setActiveView(view)}
                        isActive={isActive}
                        tooltip={source.metadata.title}
                      >
                        <Suspense
                          fallback={
                            <HelpCircle size={16} className="shrink-0" />
                          }
                        >
                          <Icon size={16} className="shrink-0" />
                        </Suspense>
                        <span>{source.metadata.title}</span>
                        <span
                          className={`ml-auto size-1 rounded-full ${
                            source.data.enabled ? "bg-green-500" : "bg-red-500"
                          }`}
                          title={source.data.enabled ? "Enabled" : "Disabled"}
                        />
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  );
                })
              ) : null}
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
