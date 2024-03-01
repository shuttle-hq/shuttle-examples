import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { Homepage } from "./home.jsx";
import { Pricing } from "./pricing.jsx";
import { Pay } from "./pay.jsx";
import { Register } from "./register.jsx";
import { Login } from "./login.jsx";
import { Dashboard } from "./dashboard.jsx";
import { PaymentSuccess } from "./success.jsx";
import { PaymentFailure } from "./failure.jsx";

import { createBrowserRouter, RouterProvider } from "react-router-dom";

const router = createBrowserRouter([
  { path: "/", element: <Homepage /> },
  { path: "/subscribe", element: <Pricing /> },
  { path: "/pay", element: <Pay /> },
  { path: "/register", element: <Register /> },
  { path: "/login", element: <Login /> },
  { path: "/dashboard", element: <Dashboard /> },
  { path: "/failure", element: <PaymentFailure /> },
  { path: "/success", element: <PaymentSuccess /> },
]);

ReactDOM.createRoot(document.getElementById("root")).render(
  <RouterProvider router={router} />,
);
