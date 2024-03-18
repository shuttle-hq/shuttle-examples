import { Header, Footer } from "./root.jsx";
import { useEffect } from "react";

export const Homepage = () => {
  return (
    <>
      <Header />
      <div className="logo">
        <h1>Loco: SaaS application</h1>
        <img
          src="https://loco.rs/images/logo.png"
          className="logo"
          alt="Loco logo"
        />
      </div>
      <Footer />
    </>
  );
};
