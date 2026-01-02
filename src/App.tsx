import { useState } from "react";
import "./App.css";
import Sidebar from "./Components/Sidebar";
import ActiveViewContent from "./Components/ActiveViewContent";
import { ViewType } from "./Components/ActiveViewContent";
import { ThemeProvider } from "./Components/theme-provider";

function App() {
  const [activeView, setActiveView] = useState<ViewType>(ViewType.Home);

  const sidebarItems = [
    { label: "Home", onClick: () => setActiveView(ViewType.Home) },
    { label: "Twitter", onClick: () => setActiveView(ViewType.Twitter) },
    { label: "BlueSky", onClick: () => setActiveView(ViewType.BlueSky) },
    { label: "Instagram", onClick: () => setActiveView(ViewType.Instagram) },
    { label: "TikTok", onClick: () => setActiveView(ViewType.TikTok) },
  ];

  return (
    <ThemeProvider defaultTheme="dark">
      <div className="flex min-h-screen bg-background">
        {/* Sidebar and Main Dock Area */}
        <div className="flex flex-col w-64 flex-shrink-0 bg-appbg text-white">
          <Sidebar items={sidebarItems} />
        </div>

        {/* Main App Section */}
        <main className="flex-grow container mx-auto px-4 py-2">
          <ActiveViewContent activeView={activeView} />
        </main>
      </div>
    </ThemeProvider>
  );
}

export default App;
