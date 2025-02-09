import StatusIndicator from "./ui/StatusIndicator";

import styles from "./StatusBar.module.scss";

const StatusBar = () => {
    return (
        <div className={styles["root"]}>
            <StatusIndicator status="green" text="Connection A" />
            <StatusIndicator status="red" text="Connection B" />
        </div>
    );
}

export default StatusBar;