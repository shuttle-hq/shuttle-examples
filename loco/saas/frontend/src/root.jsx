import { Link } from "react-router-dom";
import { useTokenStore } from "./zustand.js";

export const Header = () => {
  const tokenStore = useTokenStore();
  return (
    <header className="navbar fixed-top">
      <div className="container">
        <ul className="navbar-nav ">
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/subscribe">Pricing</Link>
          </li>

          {tokenStore.token ? (
            <li>
              <Link to="/dashboard">Dashboard</Link>
            </li>
          ) : (
            <li>
              <Link to="/login">Login</Link>
            </li>
          )}
        </ul>
      </div>
    </header>
  );
};

export const Footer = () => {
  return (
    <footer>
      <ul>
        <li>
          <a href="https://loco.rs?ref=starter" target="_blank">
            Our Documentation
          </a>
        </li>
        <li>
          <a href="https://github.com/loco-rs/loco?ref=starter" target="_blank">
            GitHub
          </a>
        </li>
        <li>
          <a
            href="https://github.com/loco-rs/loco/issues?ref=starter"
            target="_blank"
          >
            Found a bug?
          </a>
        </li>
        <li>
          <a
            href="https://github.com/loco-rs/loco/discussions?ref=starter"
            target="_blank"
          >
            Need help?
          </a>
        </li>
      </ul>
    </footer>
  );
};
