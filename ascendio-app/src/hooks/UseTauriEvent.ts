import { listen, Event } from "@tauri-apps/api/event";
import { useEffect } from "react";

import { Events } from "../models/Events";

type EventCallbackFunc<T> = (e: Event<T>) => void

export const useTauriEvent = <T>(event: Events, callback: EventCallbackFunc<T>) => {
    useEffect(() => {
        const listener = listen(event, callback);

        return () => {
            listener.then(unlistenFn => unlistenFn());
        }
    }, []);
}