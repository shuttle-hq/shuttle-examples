import { Footer } from "@/components/Footer";
import Image from "next/image";
import Link from "next/link";

export default function Home() {
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-12 row-start-2 items-center sm:items-start">
        <Image
          alt="Shuttle logo"
          src="/shuttle-logo.webp"
          width={200}
          height={200}
          className="-mt-10 self-center"
        />
        <h1 className="text-7xl font-bold"> Shuttle AI Playground </h1>
        <div className="flex flex-row justify-center w-full gap-8">
          <Link
            href="/register"
            className="bg-slate-300 text-black hover:text-slate-300 hover:bg-gradient-to-r hover:from-orange-700 hover:to-yellow-400 transition duration-300 text-2xl px-8 py-4 rounded-xl"
          >
            Register
          </Link>
          <Link
            href="/login"
            className="bg-slate-400 text-black hover:text-slate-300 hover:bg-gradient-to-r hover:from-orange-700 hover:to-yellow-400 transition duration-300 text-2xl px-8 py-4 rounded-xl"
          >
            Log In
          </Link>
        </div>
      </main>
      <Footer />
    </div>
  );
}
