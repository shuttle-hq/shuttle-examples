import { Header, Footer } from "./root.jsx";
import { useState } from "react";
import { Navigate, useNavigate, Link, redirect } from "react-router-dom";
import { useTokenStore } from "./zustand.js";

export const Login = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const state = useTokenStore();
  let navigate = useNavigate();

  async function handle_login(e) {
    e.preventDefault();

    const url =
      window.location.protocol === "https:"
        ? `https://${window.location.host}/api/auth/login`
        : `http://${window.location.host}/api/auth/login`;

    try {
      const res = await fetch(url, {
        method: "POST",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email: email,
          password: password,
        }),
      });

      if (res.ok) {
        let data = await res.json();
        state.setToken(data.token);
        console.log("Token was set");
        navigate("/dashboard");
      }
    } catch (e) {
      console.log(e.message);
    }
  }

  return (
    <>
      <Header />
      <div className="logo">
        {state.token && <Navigate to="/dashboard" replace={true} />}
        <h1>Log In</h1>
        <form className="auth-form" onSubmit={(e) => handle_login(e)}>
          <label name="email">
            <span>E-mail:</span>
            <input
              name="email"
              type="email"
              required
              placeholder="Email"
              onInput={(e) => setEmail(e.target.value)}
            />
          </label>
          <label name="password">
            <span>Password:</span>
            <input
              name="password"
              type="password"
              required
              placeholder="Password"
              onInput={(e) => setPassword(e.target.value)}
            />
          </label>
          <button type="submit" className="register-button">
            Log In
          </button>
          <Link to="/register" className="register-button">
            I don't have an account
          </Link>
        </form>
      </div>
      <Footer />
    </>
  );
};
