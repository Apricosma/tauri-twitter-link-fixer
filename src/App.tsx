import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Sidebar from "./Components/Sidebar";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const sidebarItems = [
    { label: "Home", onClick: () => alert("Home clicked") },
    { label: "Profile", onClick: () => alert("Profile clicked") },
    { label: "Settings", onClick: () => alert("Settings clicked") },
    { label: "Logout", onClick: () => alert("Logout clicked") },
  ];

  return (
    <div className="flex min-h-screen">
      <Sidebar items={sidebarItems} />
      <main className="container mx-auto p-6">
        <h1 className="text-4xl flex items-center justify-center">
          Twitter Link Fixer
        </h1>

        <div className="flex items-center justify-center space-x-6">
          <img
            alt="Twitter Link Fixer Logo"
            className="h-24 w-24 drop-shadow-xl hover:animate-spin"
            src="/twitter.svg"
          />
        </div>
      </main>
    </div>
  );
}

export default App;
