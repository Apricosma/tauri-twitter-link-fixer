import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container mx-auto p-6">
      <h1 className="text-4xl">Twitter Link Fixer</h1>

      <div className="flex items-center justify-center space-x-6 my-6">
        <img alt="React Logo" className="h-24 w-24" src={reactLogo} />
        <img alt="Vite Logo" className="h-24 w-24" src="/vite.svg" />
        <img alt="Tauri Logo" className="h-24 w-24" src="/tauri.svg" />
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button
          className="bg-sky-500 hover:bg-sky-700 py-1 px-6 rounded-2xl"
          type="submit"
        >
          Greet
        </button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
