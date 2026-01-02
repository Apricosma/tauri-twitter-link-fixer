import { useState } from "react";
import "./App.css";
import { AppSidebar } from "./components/AppSidebar";
import ActiveViewContent from "./components/ActiveViewContent";
import { ViewType } from "./components/ActiveViewContent";
import { ThemeProvider } from "./components/theme-provider";
import {
  SidebarProvider,
  SidebarInset,
  SidebarTrigger,
} from "./components/ui/sidebar";
import { Separator } from "./components/ui/separator";

function App() {
  const [activeView, setActiveView] = useState<ViewType>(ViewType.Home);

  return (
    <ThemeProvider defaultTheme="dark">
      <SidebarProvider>
        <AppSidebar activeView={activeView} setActiveView={setActiveView} />
        <SidebarInset>

          <main className="flex flex-1 flex-col gap-4 p-4 pb-0 bg-background h-screen">
            <ActiveViewContent activeView={activeView} />
          </main>
        </SidebarInset>
      </SidebarProvider>
    </ThemeProvider>
  );
}

export default App;
