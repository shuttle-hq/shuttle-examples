import { Header, Footer } from "./root.jsx";
import { useEffect } from "react";
import { Link } from "react-router-dom";
import { useTokenStore } from "./zustand.js";

export const Pricing = () => {
  let tokenStore = useTokenStore();

  return (
    <>
      <Header />
      <div className="logo">
        <h1>Subscribe</h1>
        <div className="pricing-container">
          <div className="pricing-tier">
            <h2 class="pricing-tier-header">Free</h2>
            <ul class="pricing-tier-perks">
              <li>Benefit 1</li>
              <li>Benefit 2</li>
            </ul>
            {tokenStore.token ? null : (
              <Link to="/register" class="subscribe-button">
                Get Started
              </Link>
            )}
          </div>
          <div className="pricing-tier">
            <h2 class="pricing-tier-header">Pro</h2>
            <ul class="pricing-tier-perks">
              <li>Benefit 1</li>
              <li>Benefit 2</li>
            </ul>
            {tokenStore.token ? (
              <Link to="/pay?tier=Pro" class="subscribe-button">
                Subscribe
              </Link>
            ) : (
              <Link to="/register" class="subscribe-button">
                Get Started
              </Link>
            )}
          </div>
          <div className="pricing-tier">
            <h2 class="pricing-tier-header">Team</h2>
            <ul class="pricing-tier-perks">
              <li>Benefit 1</li>
              <li>Benefit 2</li>
            </ul>
            {tokenStore.token ? (
              <Link to="/pay?tier=Team" class="subscribe-button">
                Subscribe
              </Link>
            ) : (
              <Link to="/register" class="subscribe-button">
                Get Started
              </Link>
            )}
          </div>
        </div>
      </div>
      <Footer />
    </>
  );
};
