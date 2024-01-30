"use client";
import { UserButton, useAuth } from "@clerk/nextjs";
import React from "react";
import useSWR from "swr";
import UsersTable from "./users-table";
import { Loader } from "lucide-react";

export interface UserSchema {
  first_name: string;
  last_name: string;
  username: string;
  email_addresses: { email_address: string; id: string }[];
  profile_image_url: string;
  has_image: string;
}

const getUsers = async (token: string) => {
  const res = await (
    await fetch(process.env.NEXT_PUBLIC_API_BASE_URL + "/users", {
      method: "GET",
      headers: {
        Authorization: "Bearer " + token,
      },
    })
  ).json();

  return res;
};

export default function Home() {
  const { getToken } = useAuth();

  const { isLoading, data, error } = useSWR("/api/users", async () => {
    const token = await getToken();
    return getUsers(token as string);
  });

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="fixed top-6 right-6">
        <UserButton afterSignOutUrl="/" />
      </div>
      {isLoading && <Loader className="w-4 h-4 animate-spin" />}
      {!isLoading && <UsersTable users={data.data ?? []} />}
    </main>
  );
}
