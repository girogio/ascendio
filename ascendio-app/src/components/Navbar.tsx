import { useLocation, useNavigate } from "react-router-dom";

import { Home, Settings } from "lucide-react";

import styles from "./Navbar.module.scss";

type NavitemProps = {
    label: string,
    icon: JSX.Element,
    path: string,
}

const Navbar = () => {
    return (
        <nav className={styles["root"]}>
            <Navitem label="Home" icon={<Home size={18} />} path="/" />
            <Navitem label="Settings" icon={<Settings size={18} />} path="/settings" />
        </nav>
    );
}

const Navitem = ({ label, icon, path }: NavitemProps) => {
    const navigate = useNavigate();
    const currentRoute = useLocation().pathname;

    const handleClick = () => navigate(path);

    const appliedStyles = [styles["navitem"]];

    if (currentRoute === path) {
        appliedStyles.push(styles["navitem-selected"]);
    }

    return (
        <button className={appliedStyles.join(" ")} onClick={handleClick}>{icon}{label}</button>
    )
}

export default Navbar;