import styles from "./Button.module.scss";

type ButtonProps = {
    text: string,
    onClick: () => void,
    disabled?: boolean
}

const Button = ({ text, onClick, disabled }: ButtonProps) => {
    return (
        <button className={styles["root"]} onClick={onClick} disabled={disabled}>
            {text}
        </button>
    );
}

export default Button
