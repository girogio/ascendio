import styles from "./Home.module.scss";

const Home = () => {
    return (
        <div className={styles["root"]}>
            <p style={{ fontSize: 32 }}>Hi, I'm Ascendio!</p>
        </div>
    );
}

export default Home;