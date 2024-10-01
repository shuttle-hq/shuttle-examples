import { Header, Footer } from "./root.jsx";
import { useEffect } from "react";
import { Link, useSearchParams, useNavigate } from "react-router-dom";
import React from "react";
import { useTokenStore } from "./zustand.js";

export const Pay = () => {
  const [tier, setTier] = React.useState("");
  const [price, setPrice] = React.useState("0.00");
  const tokenState = useTokenStore();
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const subscription_tier = searchParams.get("tier");

  const [name, setName] = React.useState("");
  const [email, setEmail] = React.useState("");
  const [cc1, setCc1] = React.useState("");
  const [cc2, setCc2] = React.useState("");
  const [cc3, setCc3] = React.useState("");
  const [cc4, setCc4] = React.useState("");
  const [expYear, setExpYear] = React.useState("");
  const [expMonth, setExpMonth] = React.useState("");
  const [cvc, setCvc] = React.useState("");

  async function handle_pay(e) {
    e.preventDefault();

    const url =
      window.location.protocol === "https:"
        ? `https://${window.location.host}/api/stripe/create`
        : `http://localhost:8000/api/stripe/create`;

    const combined_card_num = `${cc1}${cc2}${cc3}${cc4}`;

    try {
      const res = await fetch(url, {
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
          Authorization: `Bearer ${tokenState.token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name: name,
          email: email,
          cardNum: combined_card_num,
          expYear: Number(expYear),
          expMonth: Number(expMonth),
          cvc: cvc,
          userTier: subscription_tier,
        }),
      });

      if (res.ok) {
        navigate("/success");
      }
    } catch (e) {
      navigate("/failure");
    }
  }

  React.useEffect(() => {
    setTier(subscription_tier);
    if (tier.toLowerCase() === "pro") {
      setPrice("10.00");
    } else if (tier.toLowerCase() === "team") {
      setPrice("25.00");
    }
  }, []);

  return (
    <>
      <Header />
      <div className="logo">
        <h1>Pay</h1>
        <div className="payment-container">
          <div>
            <h2>You're purchasing:</h2>
            <h2>
              1x {tier} Subscription - £{price}/month
            </h2>
          </div>
          <form className="payment-form" onSubmit={(e) => handle_pay(e)}>
            <h2>Payment Details</h2>
            <label className="payment-label">
              <span>Full name:</span>
              <input
                type="text"
                name="name"
                onInput={(e) => setName(e.target.value)}
              />
            </label>
            <label className="payment-label">
              <span>E-mail:</span>
              <input
                type="text"
                name="name"
                onInput={(e) => setEmail(e.target.value)}
              />
            </label>
            <label className="payment-label">
              <span>Card number:</span>
              <div data-connected-inputs class="cc-inputs">
                <input
                  type="tel"
                  maxlength="4"
                  id="cc-1"
                  required
                  pattern="[0-9]{4}"
                  onInput={(e) => setCc1(e.target.value)}
                />
                <input
                  type="tel"
                  id="cc-2"
                  maxlength="4"
                  required
                  pattern="[0-9]{4}"
                  onInput={(e) => setCc2(e.target.value)}
                />
                <input
                  type="tel"
                  id="cc-3"
                  maxlength="4"
                  required
                  pattern="[0-9]{4}"
                  onInput={(e) => setCc3(e.target.value)}
                />
                <input
                  type="tel"
                  id="cc-4"
                  maxlength="4"
                  required
                  pattern="[0-9]{4}"
                  onInput={(e) => setCc4(e.target.value)}
                />
              </div>
            </label>
            <label className="payment-label">
              <span>Expiry date:</span>
              <div class="exp-inputs">
                <input
                  type="tel"
                  maxlength="2"
                  id="exp-1"
                  required
                  onInput={(e) => setExpMonth(e.target.value)}
                />
                <input
                  type="tel"
                  id="exp-2"
                  maxlength="4"
                  required
                  pattern="20[0-9]{2}"
                  onInput={(e) => setExpYear(e.target.value)}
                />
              </div>
            </label>
            <label className="payment-label">
              <span>CVC number:</span>
              <input
                type="text"
                name="cvc"
                required
                maxlength="4"
                onInput={(e) => setCvc(e.target.value)}
              />
            </label>
            <button type="submit" className="submit-payment">
              Checkout
            </button>
          </form>
        </div>
      </div>
      <Footer />
    </>
  );
};
