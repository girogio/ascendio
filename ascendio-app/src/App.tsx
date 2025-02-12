import { BrowserRouter, Route, Routes } from "react-router-dom";

import TitleBar from "./components/TitleBar";
import Navbar from "./components/Navbar";
import StatusBar from "./components/StatusBar";

import Home from "./pages/Home";
import Settings from "./pages/Settings";

import "./App.scss";
import { invoke } from "@tauri-apps/api/core";

function is_connected() {
  invoke("try_connect", {});
}

function App() {
    return (
        <div className="app-root">
            <TitleBar />
            <BrowserRouter>
                <Navbar />
                <main>
                    <Routes>
                        <Route path="/" element={<Home />} />
                        <Route path="/settings" element={<Settings />} />
                    </Routes>
                </main>
            </BrowserRouter>
            <StatusBar />
        </div>
    );
}

export default App;
