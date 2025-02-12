import { invoke } from "@tauri-apps/api/core";

export const mcuDisconnect = () => {
    return invoke<void>("disconnect");
}

export const mcuTryConnect = () => {
    return invoke<void>("try_connect");
}