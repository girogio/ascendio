import { useEffect, useState, useCallback } from "react";

import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { UnlistenFn } from "@tauri-apps/api/event";

import { Minimize, Minus, Square, X } from "lucide-react";

import styles from "./WindowControls.module.scss";

const appWindow = getCurrentWebviewWindow()

const WindowControls = () => {
    const [isMaximized, setIsMaximized] = useState<boolean>();

    const updateIsMaximized = useCallback(async () => setIsMaximized(await appWindow.isMaximized()), []);

    useEffect(() => {
        updateIsMaximized();

        let unlisten: UnlistenFn;

        const listen = async () => {
            unlisten = await appWindow.onResized(() => {
                updateIsMaximized();
            });
        };

        listen();

        return () => unlisten && unlisten();
    }, [updateIsMaximized]);

    return (
        <div className={styles["root"]}>
            <button className={[styles["btn"], styles["btn-anim"]].join(" ")} onClick={() => appWindow.minimize()}><Minus color="#fff" size={16} /></button>
            <button className={[styles["btn"], styles["btn-anim"]].join(" ")} onClick={() => appWindow.toggleMaximize()}>{isMaximized ? <Minimize color="#fff" size={14} /> : <Square color="#fff" size={14} />}</button>
            <button className={[styles["btn"], styles["btn-anim"], styles["close"], styles["close-anim"]].join(" ")} onClick={() => appWindow.close()}><X color="#fff" size={18} /></button>
        </div>
    );
}

export default WindowControls;