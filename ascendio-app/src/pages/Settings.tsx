import styles from "./Settings.module.scss";

const Settings = () => {
    return (
        <div className={styles["root"]}>
            <p style={{ fontSize: 32 }}>This is the <b>Settings</b> page.</p>
        </div>
    );
}

export default Settings;