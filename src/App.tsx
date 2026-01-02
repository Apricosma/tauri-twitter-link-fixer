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
} from "./components/ui/sidebar";

function App() {
  const [activeView, setActiveView] = useState<ViewType>(ViewType.Home);

  return (
    <ThemeProvider defaultTheme="dark">
      <SidebarProvider>
        <AppSidebar activeView={activeView} setActiveView={setActiveView} />
        <SidebarInset>
          <main className="flex flex-1 flex-col gap-4 p-4 pb-0 bg-background h-screen overflow-hidden">
            <div className="flex-1 min-h-0">
              <ActiveViewContent activeView={activeView} />
            </div>
          </main>
        </SidebarInset>
      </SidebarProvider>
      <Toaster position="bottom-right" />
    </ThemeProvider>
  );
}

export default App;
