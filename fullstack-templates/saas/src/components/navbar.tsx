import React, { useState } from 'react';
import Link from 'next/link';
import { accountStore } from '@/stores/zustandStore';
import { useRouter } from 'next/router';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faBars,
  faCoins,
  faGauge,
  faHandshake,
  faRightFromBracket,
  faRocket,
  faUserSecret,
  faUsers,
} from '@fortawesome/free-solid-svg-icons';
import Navigation from './Navigation';

export default function Navbar() {
  const { email, changeEmail } = accountStore();
  const router = useRouter();
  const [open, setOpen] = useState(false);

  const handleLogout = async () => {
    const url = `//${window.location.host}/api/auth/logout`;

    const res = await fetch(url);

    if (res.ok) {
      changeEmail('');
      router.push('/');
    }
  };

  if (email === '') return <Navigation />;

  return (
    <>
      <aside
        className={`flex flex-col w-full relative z-10 lg:w-64 lg:h-screen px-5 py-8 overflow-y-auto bg-white border-r rtl:border-r-0 rtl:border-l dark:bg-gray-900 dark:border-gray-700 ${
          open && 'h-screen'
        }`}
      >
        <button className="absolute right-6 dark:text-[#D8D8D8] lg:hidden" onClick={() => setOpen((open) => !open)}>
          <FontAwesomeIcon icon={faBars} color="black" size="2x" fixedWidth />
        </button>
        <Link href="/">
          <FontAwesomeIcon icon={faRocket} size="2x" color="black" />
        </Link>
        <div className={`lg:flex flex-col justify-between flex-1 mt-6 ${!open && 'hidden'}`}>
          <nav className="-mx-3 space-y-3 lg:space-y-6 flex flex-col lg:justify-between lg:h-full">
            <div className="space-y-3">
              <Link
                className="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
                href="/dashboard"
              >
                <FontAwesomeIcon icon={faGauge} color="black" fixedWidth />

                <span className="mx-2 text-sm font-medium">Dashboard</span>
              </Link>

              <Link
                className="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
                href="/dashboard/customers"
              >
                <FontAwesomeIcon icon={faUsers} color="black" fixedWidth />

                <span className="mx-2 text-sm font-medium">Customers</span>
              </Link>
              <Link
                className="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
                href="/dashboard/deals"
              >
                <FontAwesomeIcon icon={faHandshake} color="black" fixedWidth />

                <span className="mx-2 text-sm font-medium">Deals</span>
              </Link>
              <Link
                className="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
                href="/dashboard/upgrade"
              >
                <FontAwesomeIcon icon={faCoins} color="black" fixedWidth />

                <span className="mx-2 text-sm font-medium">Upgrade</span>
              </Link>
            </div>

            <div className="space-y-3 flex flex-col dark:text-gray-200 items-start">
              <div className="px-3">
                <FontAwesomeIcon icon={faUserSecret} color="black" fixedWidth />
                <span className="text-sm mx-2 font-medium">{email}</span>
              </div>
              <div className="px-3">
                <FontAwesomeIcon icon={faRightFromBracket} color="black" fixedWidth />
                <button className="text-sm mx-2 font-medium" onClick={handleLogout}>
                  Log Out
                </button>
              </div>
            </div>
          </nav>
        </div>
      </aside>
    </>
  );
}
