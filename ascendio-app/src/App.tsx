import TitleBar from "./components/TitleBar";
import StatusBar from "./components/StatusBar";

import "./App.scss";
import { invoke } from "@tauri-apps/api/core";

function is_connected() {
  invoke("try_connect", {});
}

function App() {
  return (
    <div className="app-root">
      <TitleBar />
      <main>
        <p style={{ fontSize: 32 }}>Hi, I'm Ascendio!</p>
      </main>
      <StatusBar />
    </div>
  );
}

export default App;
