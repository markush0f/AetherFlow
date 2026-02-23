import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/Sidebar";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="flex min-h-screen bg-brand-black text-white selection:bg-brand-orange selection:text-brand-black">
      <Sidebar />
      <main className="flex-1 flex flex-col items-center justify-center p-12">
        <div className="max-w-2xl w-full space-y-24">

          <div className="space-y-12 text-center flex flex-col items-center">
            <img 
              src="/logo.png" 
              alt="AetherFlow Logo" 
              className="h-48 w-auto opacity-100 transition-transform duration-700 hover:scale-105 contrast-110 drop-shadow-[0_0_30px_rgba(255,107,0,0.15)]"
            />
            <div className="space-y-6">
              <h1 className="text-4xl font-light tracking-[0.3em] text-white/90 uppercase">
                Aether<span className="text-brand-orange">Flow</span>
              </h1>
              <div className="h-px w-12 bg-brand-blue/30 mx-auto" />
              <p className="text-gray-500 font-light tracking-widest text-[10px] uppercase">
                Developed with <span className="text-brand-blue">Tauri</span> + React
              </p>
            </div>
          </div>

          <div className="max-w-md mx-auto w-full space-y-12">
            <form
              className="group relative"
              onSubmit={(e) => {
                e.preventDefault();
                greet();
              }}
            >
              <input
                id="greet-input"
                className="w-full bg-transparent border-b border-white/10 py-4 px-0 outline-none focus:border-brand-orange transition-colors duration-500 font-light placeholder:text-gray-700 text-sm tracking-widest uppercase"
                onChange={(e) => setName(e.currentTarget.value)}
                placeholder="Enter Identity"
              />
              <button
                type="submit"
                className="absolute right-0 bottom-4 text-[10px] text-gray-500 hover:text-brand-orange transition-colors uppercase tracking-[0.2em] font-medium"
              >
                Invoke
              </button>
            </form>

            {greetMsg && (
              <div className="text-center animate-in fade-in slide-in-from-top-2 duration-700">
                <span className="text-xs font-light text-brand-blue tracking-[0.15em]">
                  {greetMsg}
                </span>
              </div>
            )}
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
