import { BrowserRouter, Route, Routes } from "react-router-dom";

import TitleBar from "./components/TitleBar";
import StatusBar from "./components/StatusBar";

import Home from "./pages/Home";
import Settings from "./pages/Settings";

import "./App.scss";

function App() {
    return (
        <div className="app-root">
            <TitleBar />
            <main>
                <BrowserRouter>
                    <Routes>
                        <Route path="/" element={<Home />} />
                        <Route path="/settings" element={<Settings />} />
                    </Routes>
                </BrowserRouter>
            </main>
            <StatusBar />
        </div>
    );
}

export default App;
