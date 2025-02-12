import { ButtonHTMLAttributes, PropsWithChildren } from "react";

import styles from "./Button.module.scss";

const Button = ({ children, ...props }: PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>>) => {
    return (
        <button className={[props.className, styles["root"]].join(" ")} {...props}>
            {children}
        </button>
    );
}

export default Button;
