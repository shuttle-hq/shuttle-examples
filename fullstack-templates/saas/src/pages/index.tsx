import Layout from '../components/Layout';
import React from 'react';

const cards = [
  {
    title: 'Deployed from Shuttle',
    description: 'Deployed via Shuttle, a Rust-native cloud dev platform.',
    href: 'https://www.shuttle.rs',
  },
  {
    title: 'Learn About Rust',
    description: 'Rust is a brilliant language for writing memory-safe, efficient software.',
    href: 'https://doc.rust-lang.org/book/',
  },
  {
    title: 'Learn About Next.js',
    description: 'Next.js is a React-based meta-framework at the forefront of JavaScript.',
    href: 'https://nextjs.org/',
  },
  {
    title: 'Join the community',
    description: 'Join a thriving community with like-minded Rustaceans and web developers.',
    href: 'https://discord.com/invite/shuttle',
  },
];

export default function Home() {
  return (
    <>
      <Layout>
        <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
          <h1 className="text-5xl font-bold text-center px-5">
            Welcome to <span className="text-[#EF924C]">Next.js + Rust SaaS Template</span>
          </h1>

          <div className="px-5 lg:w-1/2 grid grid-cols-1 sm:grid-cols-2 justify-center gap-6">
            {cards.map((card) => (
              <div
                key={card.title}
                className="px-10 py-5 rounded-lg border h-full flex flex-col justify-between space-y-2"
              >
                <h2 className="font-bold text-xl">{card.title} &rarr;</h2>
                <p>{card.description}</p>
                <a href={card.href} target="_blank" className="text-[#EF924C] transition-all hover:underline">
                  Learn more here
                </a>
              </div>
            ))}
          </div>

          <p className="text-xs">
            Powered by{' '}
            <a className="font-semibold" href="https://shuttle.rs" target="_blank">
              🚀 Shuttle
            </a>
          </p>
        </section>
      </Layout>
    </>
  );
}
