import { useState } from "react";
import "./App.css";
import { AppSidebar } from "./components/AppSidebar";
import ActiveViewContent from "./components/ActiveViewContent";
import { ViewType } from "./components/ActiveViewContent";
import { ThemeProvider } from "./components/theme-provider";
import { Toaster } from "./components/ui/sonner";
import {
  SidebarProvider,
  SidebarInset,
  SidebarTrigger,
} from "./components/ui/sidebar";

function App() {
  const [activeView, setActiveView] = useState<ViewType>("home");

  return (
    <ThemeProvider defaultTheme="dark">
      <SidebarProvider>
        <AppSidebar activeView={activeView} setActiveView={setActiveView} />
        <SidebarInset>
          <header className="flex h-12 bg-sidebar shrink-0 items-center gap-2 border-b px-4 md:hidden">
            <SidebarTrigger />
            <div className="flex items-center gap-2">
              <span className="text-lg">
                <img src="/appLogo.png" alt="App Logo" className="h-8 w-8" />
              </span>
              <span className="font-semibold">Cosma Link Converter</span>
            </div>
          </header>
          <main className="flex flex-1 flex-col gap-4 p-4 pb-0 bg-background h-screen overflow-hidden">
            <div className="flex-1 min-h-0">
              <ActiveViewContent
                activeView={activeView}
                setActiveView={setActiveView}
              />
            </div>
          </main>
        </SidebarInset>
      </SidebarProvider>
      <Toaster position="bottom-right" />
    </ThemeProvider>
  );
}

export default App;
