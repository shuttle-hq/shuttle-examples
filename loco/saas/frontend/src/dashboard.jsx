import { Header, Footer } from "./root.jsx";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useTokenStore } from "./zustand.js";

export const Dashboard = () => {
  const tokenStore = useTokenStore();
  let navigate = useNavigate();

  async function cancelSubscription() {
    const url =
      window.location.protocol === "https:"
        ? `https://${window.location.host}/api/stripe/cancel`
        : `http://${window.location.host}/api/stripe/cancel`;

    const res = await fetch(url, {
      method: "DELETE",
      mode: "cors",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${tokenStore.token}`,
      },
    });

    if (res.ok) {
      alert("Subscription has been cancelled!");
      navigate("/dashboard");
    } else {
      alert(`Subscription could not be cancelled :(`);
    }
  }

  async function updateSubscription(tier) {
    const url =
      window.location.protocol === "https:"
        ? `https://${window.location.host}/api/stripe/update`
        : `http://${window.location.host}/api/stripe/update`;

    const res = await fetch(url, {
      method: "POST",
      mode: "cors",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${tokenStore.token}`,
      },
      body: JSON.stringify({
        userTier: tier,
      }),
    });

    if (res.ok) {
      alert("Subscription has been updated!");
      navigate("/dashboard");
    } else {
      alert(`Subscription could not be updated :(`);
    }
  }

  useEffect(() => {
    async function get_user_tier() {
      if (tokenStore.token === "") {
        navigate("/login");
        return;
      }

      const url =
        window.location.protocol === "https:"
          ? `https://${window.location.host}/api/stripe/get_current_tier`
          : `http://${window.location.host}/api/stripe/get_current_tier`;

      const res = await fetch(url, {
        mode: "cors",
        method: "GET",
        headers: {
          Authorization: `Bearer ${tokenStore.token}`,
        },
      });

      if (res.ok) {
        const data = await res.json();
        tokenStore.setTier(data.userTier.toLowerCase());
      }
    }
    get_user_tier();
  }, []);

  return (
    <>
      <Header />
      <div className="logo">
        <h1>Dashboard</h1>
        <p>Hey there! You're logged in.</p>
        {tokenStore.tier !== "free" ? null : (
          <button onClick={cancelSubscription}>Cancel Subscription</button>
        )}
        {tokenStore.tier === "pro" ? (
          <button onClick={() => updateSubscription("Team")}>
            Upgrade to Team Tier
          </button>
        ) : null}
        {tokenStore.tier === "team" ? (
          <button onClick={() => updateSubscription("Pro")}>
            Downgrade to Pro Tier
          </button>
        ) : null}
      </div>
      <Footer />
    </>
  );
};
