import Navbar from './navbar';
import React from 'react';
import Head from 'next/head';
import { accountStore } from '@/stores/zustandStore';
import { useRouter } from 'next/router';

type Props = {
  children: React.ReactNode | React.ReactNode[];
};

export default function Layout({ children }: Props) {
  let router = useRouter();
  let { email } = accountStore();

  React.useEffect(() => {
    if (router.pathname.includes(`${window.location.host}/dashboard`) && email == '') {
      router.push('/login');
    }
  }, [email, router, router.pathname]);

  return (
    <div className="min-h-screen flex-col items-center">
      <Navbar />
      {children}
    </div>
  );
}
