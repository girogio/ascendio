import { Check, X } from "lucide-react";

import styles from "./StatusIndicator.module.scss";

type StatusIndicatorProps = {
    status: "green" | "red",
    text: string
}

type StatusIndicatorCircleProps = {
    status: "green" | "red"
}

const StatusIndicator = ({ status, text }: StatusIndicatorProps) => {
    return (
        <div className={styles["root"]}>
            <p>{text}</p>
            <StatusIndicatorCircle status={status} />
        </div>
    );
}

const StatusIndicatorCircle = ({ status }: StatusIndicatorCircleProps) => {
    return (
        <div className={styles["circle-root"]} style={{ backgroundColor: status === "green" ? "var(--status-green)" : "var(--status-red)" }}>
            {status === "green" ? <Check size={10} /> : <X size={10} />}
        </div>
    );
}

export default StatusIndicator;