import { Header, Footer } from "./root.jsx";
import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";

export const Register = () => {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  let router = useNavigate();

  async function handle_register(e) {
    e.preventDefault();

    const url =
      window.location.protocol === "https:"
        ? `https://${window.location.host}/api/auth/register`
        : `http://${window.location.host}/api/auth/register`;

    let res = await fetch(url, {
      mode: "cors",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: username,
        email: email,
        password: password,
      }),
    });

    if (res.created) {
      router.navigate("/login");
    }
  }

  return (
    <>
      <Header />
      <div className="logo">
        <h1>Register</h1>
        <form className="auth-form" onSubmit={(e) => handle_register(e)}>
          <label name="username">
            <span>Username:</span>
            <input
              name="username"
              type="text"
              required
              placeholder="Username"
              onInput={(e) => setUsername(e.target.value)}
            />
          </label>
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
            Register
          </button>
          <Link to="/login" className="register-button">
            I already have an account
          </Link>
        </form>
      </div>
      <Footer />
    </>
  );
};
