"use client";
import { auth } from "@clerk/nextjs";
import React from "react";
import useSWR from "swr";

const getUsers = async (sessionId: string) => {
  const res = await (
    await fetch(process.env.NEXT_PUBLIC_API_BASE_URL + "/users", {
      credentials: "include",
      headers: {
        "X-CLERK-SESSION_ID": sessionId,
      },
    })
  ).json();
  console.log(res);

  return res;
};

export default async function Home() {
  const { sessionId } = auth();
  // const [users, setUsers] = React.useState();
  const { isLoading, data, error } = useSWR("/api/users", () =>
    getUsers(sessionId as string)
  );
  console.log(error, data, isLoading);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {isLoading ? "Loading" : "Data"}
    </main>
  );
}
