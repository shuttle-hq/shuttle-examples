import Navbar from './navbar';
import React from 'react';
import Head from 'next/head';
import { accountStore } from '@/stores/zustandStore';

type Props = {
  children: React.ReactNode | React.ReactNode[];
};

export default function Layout({ children }: Props) {
  let { email } = accountStore();

  return (
    <>
      <Head>
        <title>Shuttle SaaS</title>
      </Head>
      <div
        className={
          email === ''
            ? 'min-h-screen flex-col items-center'
            : 'min-h-screen w-screen flex flex-col lg:flex-row items-top'
        }
      >
        <Navbar />
        <div className={`relative bg-slate-50 h-screen overflow-y-scroll ${email ? 'w-full' : 'w-screen'}`}>
          {children}
        </div>
      </div>
    </>
  );
}
