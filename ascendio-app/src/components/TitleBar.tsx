import WindowControls from "./ui/WindowControls";

import styles from "./TitleBar.module.scss";

const TitleBar = () => {
    return (
        <div data-tauri-drag-region className={styles["root"]}>
            <p className={styles["wordmark"]}>Ascendio</p>
            <WindowControls />
        </div>
    );
}

export default TitleBar;