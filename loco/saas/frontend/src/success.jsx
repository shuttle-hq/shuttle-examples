import { Header, Footer } from "./root.jsx";
import { useEffect } from "react";

export const PaymentSuccess = () => {
  return (
    <>
      <Header />
      <div className="logo">
        <h1>Payment successful!</h1>
        <p>
          Thanks for subscribing! We hope you're empowered to be even more
          productive.
        </p>
      </div>
      <Footer />
    </>
  );
};
