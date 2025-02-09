import TitleBar from "./components/TitleBar";
import StatusBar from "./components/StatusBar";

import "./App.scss";

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
