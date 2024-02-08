import Layout from '../components/Layout';
import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';
import Link from 'next/link';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faAt, faEye, faEyeSlash, faLock } from '@fortawesome/free-solid-svg-icons';

export default function Home() {
    const [loginEmail, setLoginEmail] = React.useState<string>('');
    const [pw, setPw] = React.useState<string>('');
    const [pwVis, setPwVis] = React.useState<boolean>(false);

    const {email, changeEmail} = accountStore();

    let router = useRouter();

    const handleSubmit = async (e: React.SyntheticEvent) => {
        e.preventDefault();

        const url = `//${window.location.host}/api/auth/login`;

        try {
            let res = await fetch(url, {
                method: 'POST',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    email: loginEmail,
                    password: pw,
                }),
            });

            if (res.ok) {
                changeEmail(loginEmail);
                router.push('/dashboard');
            } else {
                console.log('Incorrect login details.');
            }
        } catch (e: any) {
            console.log(`Error: ${e}`);
        }
    };

    const togglePassword = (e: React.SyntheticEvent) => {
        e.preventDefault();

        setPwVis(!pwVis);
    };

    return (
        <>
            <Layout>
                <form
                    className="px-5 min-h-screen flex flex-col items-center justify-center bg-gray-100"
                    onSubmit={handleSubmit}
                >
                    <div
                        className="
          flex flex-col
          bg-white
          shadow-md
          px-4
          sm:px-6
          md:px-8
          lg:px-10
          py-8
          rounded-md
          w-50
          max-w-md"
                    >
                        <h1 className="lg:text-2xl text-xl text-center font-bold">Welcome Back</h1>
                        <fieldset className="mt-10">
                            <label htmlFor="email" className="text-xs tracking-wide text-gray-600">
                                E-Mail Address:
                            </label>
                            <div className="relative mb-4">
                                <FontAwesomeIcon
                                    className="inline-flex
                    items-center
                    justify-center
                    absolute
                    left-3
                    top-[30%]
                    h-full"
                                    icon={faAt}
                                    color="black"
                                />
                                <input
                                    type="email"
                                    name="email"
                                    className="text-sm
                    placeholder-gray-500
                    pl-10
                    pr-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                                    value={loginEmail}
                                    onInput={(e) => setLoginEmail((e.target as HTMLInputElement).value)}
                                    placeholder="Enter your email"
                                />
                            </div>
                            <label htmlFor="password" className="text-xs tracking-wide text-gray-600">
                                Password:
                            </label>

                            <div className="relative mb-4">
                                <FontAwesomeIcon
                                    className="inline-flex
                    items-center
                    justify-center
                    absolute
                    left-3
                    top-[30%]
                    h-full"
                                    icon={faLock}
                                    color="black"
                                />

                                <input
                                    type={pwVis ? 'text' : 'password'}
                                    name="password"
                                    className="text-sm
                    placeholder-gray-500
                    pl-10
                    pr-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                                    value={pw}
                                    onInput={(e) => setPw((e.target as HTMLInputElement).value)}
                                    placeholder="Enter your password"
                                />
                                <FontAwesomeIcon
                                    className="inline-flex
                    items-center
                    justify-center
                    absolute
                    right-3
                    top-[30%]
                    h-full"
                                    icon={pwVis ? faEyeSlash : faEye}
                                    onClick={togglePassword}
                                    color="black"
                                />
                            </div>

                            <div className="flex w-full">
                                <button
                                    type="submit"
                                    className="
                  flex
                  mt-2
                  items-center
                  justify-center
                  focus:outline-none
                  text-white text-sm
                  sm:text-base
                  bg-black
                  hover:bg-slate-950
                  rounded-md
                  py-2
                  w-full
                  transition
                  duration-150
                  ease-in
                "
                                >
                                    <span className="mr-2 uppercase">Sign In &rarr;</span>
                                </button>
                            </div>
                        </fieldset>
                    </div>
                    <div className="flex justify-center items-center mt-6">
            <span
                className=" inline-flex
            items-center
            text-gray-700
            font-medium
            text-xs text-center"
            >
              Don&apos;t have an account?
              <Link href="/register" className="text-xs ml-2 text-black font-semibold">
                Register now
              </Link>
            </span>
                    </div>
                </form>
            </Layout>
        </>
    );
}
