"use client";
import { auth, useAuth } from "@clerk/nextjs";
import React from "react";
import useSWR from "swr";

const getUsers = async (token: string) => {
  const res = await fetch(process.env.NEXT_PUBLIC_API_BASE_URL + "/users", {
    method: "GET",
    headers: {
      Authorization: "Bearer " + token,
    },
  });
  return res.json();
};

export default function Home() {
  const { getToken } = useAuth();

  const { isLoading, data, error } = useSWR("/api/users", async () => {
    const token = await getToken();
    getUsers(token as string);
  });
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {isLoading ? "Loading" : "Data"}
    </main>
  );
}
