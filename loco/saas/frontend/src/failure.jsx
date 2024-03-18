import { Header, Footer } from "./root.jsx";
import { useEffect } from "react";

export const PaymentFailure = () => {
  return (
    <>
      <Header />
      <div className="logo">
        <h1>Payment unsuccessful :( </h1>
        <p>Unfortunately, your payment wasn't successful.</p>
      </div>
      <Footer />
    </>
  );
};
