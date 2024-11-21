"use client";
import { Footer } from "@/components/Footer";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function Register() {
  const router = useRouter();

  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const res = await fetch("/api/auth/register", {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify({
        username: username,
        password: password,
      }),
    });

    if (res.ok) {
      router.replace("/login");
    }
  }

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <form
          className="flex flex-col items-center gap-8"
          onSubmit={(e) => handleSubmit(e)}
        >
          <h1 className="text-3xl font-bold"> Register </h1>
          <div className="grid grid-rows-auto grid-cols-auto gap-8">
            <label className="grid grid-rows-1 grid-cols-6 text-center align-text-bottom">
              <span className="col-span-2 row-span-auto mt-2 font-bold">
                Username:
              </span>
              <input
                name="username"
                type="text"
                placeholder="Username"
                className="px-4 py-2 text-black col-span-4 row-span-auto"
                value={username}
                onInput={(e) =>
                  setUsername((e.target as HTMLInputElement).value)
                }
                required
              ></input>
            </label>
            <label className="grid grid-rows-auto grid-cols-6 text-center align-text-bottom">
              <span className="col-span-2 text-center align-text-bottom mt-2 font-bold">
                Password:
              </span>
              <input
                name="password"
                type="password"
                placeholder="Password"
                required
                className="px-4 py-2 text-black col-span-4"
                value={password}
                onInput={(e) =>
                  setPassword((e.target as HTMLInputElement).value)
                }
              ></input>
            </label>
            <button
              type="submit"
              className="bg-slate-300 text-black hover:text-slate-300 hover:bg-gradient-to-r hover:from-orange-700 hover:to-yellow-400 transition duration-300 text-lg px-4 py-2 rounded-xl"
            >
              Submit
            </button>
          </div>
          <div className="flex flex-row justify-center w-full gap-8">
            <Link
              href="/login"
              className="w-full text-center bg-slate-300 text-black hover:text-slate-300 hover:bg-gradient-to-r hover:from-orange-700 hover:to-yellow-400 transition duration-300 text-lg px-4 py-2 rounded-xl"
            >
              I already have an account
            </Link>
          </div>
        </form>
      </main>
      <Footer />
    </div>
  );
}
