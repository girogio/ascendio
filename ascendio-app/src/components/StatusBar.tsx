import { useState } from "react";

import { useTauriEvent } from "../hooks/UseTauriEvent";

import { Events } from "../models/Events";

import StatusIndicator from "./ui/StatusIndicator";

import styles from "./StatusBar.module.scss";

const StatusBar = () => {
    const [mcuConnected, setMCUConnected] = useState<boolean>(false);
    const [simConnectConnected, setSimConnectConnected] = useState<boolean>(false);

    useTauriEvent(Events.ASCENDIO_MCU_CONNECTED, () => setMCUConnected(true));
    useTauriEvent(Events.ASCENDIO_MCU_DISCONNECTED, () => setMCUConnected(false));
    useTauriEvent(Events.ASCENDIO_SIMCONNECT_CONNECTED, () => setSimConnectConnected(true));
    useTauriEvent(Events.ASCENDIO_SIMCONNECT_DISCONNECTED, () => setSimConnectConnected(false));

    return (
        <div className={styles["root"]}>
            <StatusIndicator status={mcuConnected ? "green" : "red"} text="MCU" />
            <StatusIndicator status={simConnectConnected ? "green" : "red"} text="SimConnect" />
        </div>
    );
}

export default StatusBar;